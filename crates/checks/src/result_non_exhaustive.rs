//! Detects match expressions over `Result` values that handle `Ok` without an explicit `Err` arm.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMatch, File, Pat};

const CHECK_NAME: &str = "result-non-exhaustive";

pub struct ResultNonExhaustiveCheck;

impl Check for ResultNonExhaustiveCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut scanner = MatchScanner {
                fn_name: fn_name.clone(),
                out: &mut out,
            };
            scanner.visit_block(&method.block);
        }
        out
    }
}

struct MatchScanner<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for MatchScanner<'_> {
    fn visit_expr_match(&mut self, i: &'ast ExprMatch) {
        let mut has_ok = false;
        let mut has_err = false;
        let mut has_wildcard = false;

        for arm in &i.arms {
            if pat_contains_variant(&arm.pat, "Ok") {
                has_ok = true;
            }
            if pat_contains_variant(&arm.pat, "Err") {
                has_err = true;
            }
            if pat_is_wildcard(&arm.pat) {
                has_wildcard = true;
            }
        }

        if has_ok && !has_err && !has_wildcard {
            self.out.push(Finding {
                check_name: CHECK_NAME.to_string(),
                severity: Severity::Medium,
                file_path: String::new(),
                line: i.match_token.span().start().line,
                function_name: self.fn_name.clone(),
                description: format!(
                    "Method `{}` matches a `Result` and handles `Ok(...)` without an explicit `Err(...)` arm. "
                    "This can panic or ignore errors when the result is `Err`.",
                    self.fn_name
                ),
            });
        }

        visit::visit_expr_match(self, i);
    }
}

fn pat_contains_variant(pat: &Pat, variant: &str) -> bool {
    match pat {
        Pat::TupleStruct(ts) => path_is_variant(&ts.path, variant),
        Pat::Path(p) => path_is_variant(&p.path, variant),
        Pat::Struct(s) => path_is_variant(&s.path, variant),
        Pat::Or(o) => o.cases.iter().any(|case| pat_contains_variant(case, variant)),
        Pat::Reference(r) => pat_contains_variant(&r.pat, variant),
        Pat::Box(b) => pat_contains_variant(&b.pat, variant),
        _ => false,
    }
}

fn path_is_variant(path: &syn::Path, variant: &str) -> bool {
    path.segments
        .last()
        .is_some_and(|seg| seg.ident == variant)
}

fn pat_is_wildcard(pat: &Pat) -> bool {
    matches!(pat, Pat::Wild(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    fn run_on_src(src: &str) -> Result<Vec<Finding>, syn::Error> {
        let file = parse_file(src)?;
        Ok(ResultNonExhaustiveCheck.run(&file, src))
    }

    #[test]
    fn flags_match_with_only_ok_arm() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn run(env: Env) -> i128 {
        let result: Result<i128, ()> = Err(());
        match result {
            Ok(v) => v
        }
    }
}
"#,
        )?;
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::Medium);
        assert_eq!(hits[0].check_name, CHECK_NAME);
        Ok(())
    }

    #[test]
    fn does_not_flag_when_err_arm_present() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn run(env: Env) -> i128 {
        let result: Result<i128, ()> = Err(());
        match result {
            Ok(v) => v,
            Err(_) => 0,
        }
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn does_not_flag_when_wildcard_arm_present() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn run(env: Env) -> i128 {
        let result: Result<i128, ()> = Err(());
        match result {
            Ok(v) => v,
            _ => 0,
        }
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn ignores_non_contractimpl_matches() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{Env};

pub fn helper(env: Env) -> i128 {
    let result: Result<i128, ()> = Err(());
    match result {
        Ok(v) => v
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }
}
