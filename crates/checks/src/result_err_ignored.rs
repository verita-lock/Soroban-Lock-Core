//! Flags `if let Ok(_) = expr { ... }` without an `else` branch in `#[contractimpl]` functions.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{ExprIf, File, Pat, PatTupleStruct};

const CHECK_NAME: &str = "result-err-ignored";

pub struct ResultErrIgnoredCheck;

impl Check for ResultErrIgnoredCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut v = Visitor {
                fn_name: fn_name.clone(),
                out: &mut out,
            };
            v.visit_block(&method.block);
        }
        out
    }
}

/// Returns true if the pattern is `Ok(...)`.
fn pat_is_ok(pat: &Pat) -> bool {
    if let Pat::TupleStruct(PatTupleStruct { path, .. }) = pat {
        path.segments.last().is_some_and(|s| s.ident == "Ok")
    } else {
        false
    }
}

struct Visitor<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for Visitor<'_> {
    fn visit_expr_if(&mut self, i: &'ast ExprIf) {
        // Check for `if let Ok(...) = expr` with no else branch
        if i.else_branch.is_none() {
            if let syn::Expr::Let(expr_let) = &*i.cond {
                if pat_is_ok(&expr_let.pat) {
                    self.out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::Low,
                        file_path: String::new(),
                        line: i.span().start().line,
                        function_name: self.fn_name.clone(),
                        description: format!(
                            "`if let Ok(...)` in `{}` has no `else` branch — errors are \
                             silently ignored. Use `match`, `?`, or an explicit `else` to \
                             handle the `Err` variant.",
                            self.fn_name
                        ),
                    });
                }
            }
        }
        // Recurse into nested expressions
        visit::visit_expr_if(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    fn run(src: &str) -> Vec<Finding> {
        let file = parse_file(src).expect("parse");
        ResultErrIgnoredCheck.run(&file, src)
    }

    #[test]
    fn flags_if_let_ok_no_else() {
        let hits = run(r#"
use soroban_sdk::{contractimpl, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn transfer(env: Env) {
        if let Ok(val) = some_op() {
            env.storage().instance().set(&"k", &val);
        }
    }
}
"#);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].function_name, "transfer");
        assert_eq!(hits[0].severity, Severity::Low);
        assert_eq!(hits[0].check_name, CHECK_NAME);
    }

    #[test]
    fn passes_if_let_ok_with_else() {
        let hits = run(r#"
use soroban_sdk::{contractimpl, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn transfer(env: Env) {
        if let Ok(val) = some_op() {
            env.storage().instance().set(&"k", &val);
        } else {
            panic!("error");
        }
    }
}
"#);
        assert!(hits.is_empty());
    }

    #[test]
    fn passes_match_on_result() {
        let hits = run(r#"
use soroban_sdk::{contractimpl, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn transfer(env: Env) {
        match some_op() {
            Ok(val) => { let _ = val; }
            Err(_) => {}
        }
    }
}
"#);
        assert!(hits.is_empty());
    }

    #[test]
    fn ignores_non_contractimpl() {
        let hits = run(r#"
pub struct C;
impl C {
    pub fn helper() {
        if let Ok(val) = some_op() {
            let _ = val;
        }
    }
}
"#);
        assert!(hits.is_empty());
    }

    #[test]
    fn flags_multiple_occurrences() {
        let hits = run(r#"
use soroban_sdk::{contractimpl, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn foo(env: Env) {
        if let Ok(a) = op1() { let _ = a; }
        if let Ok(b) = op2() { let _ = b; }
    }
}
"#);
        assert_eq!(hits.len(), 2);
    }
}
