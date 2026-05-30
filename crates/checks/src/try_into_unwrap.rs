//! Flags `.unwrap()` on `try_into()` results, which can panic on type conversion failure.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File};

const CHECK_NAME: &str = "try-into-unwrap";

/// Flags `.try_into().unwrap()` in `#[contractimpl]` methods.
/// `try_into()` returns a `Result`; calling `.unwrap()` on conversion failure panics.
pub struct TryIntoUnwrapCheck;

impl Check for TryIntoUnwrapCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut v = TryIntoVisitor {
                fn_name: fn_name.clone(),
                out: &mut out,
            };
            v.visit_block(&method.block);
        }
        out
    }
}

fn is_try_into_unwrap(m: &ExprMethodCall) -> bool {
    if m.method != "unwrap" {
        return false;
    }
    matches!(&*m.receiver, Expr::MethodCall(inner) if inner.method == "try_into")
}

struct TryIntoVisitor<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
}

impl Visit<'_> for TryIntoVisitor<'_> {
    fn visit_expr_method_call(&mut self, i: &ExprMethodCall) {
        if is_try_into_unwrap(i) {
            self.out.push(Finding {
                check_name: CHECK_NAME.to_string(),
                severity: Severity::Low,
                file_path: String::new(),
                line: i.span().start().line,
                function_name: self.fn_name.clone(),
                description: format!(
                    "`{}` calls `.try_into().unwrap()`. \
                     `try_into()` returns `Result` and panics on conversion failure. \
                     Use `unwrap_or`, `unwrap_or_else`, explicit error handling, or check the conversion first.",
                    self.fn_name
                ),
            });
        }
        visit::visit_expr_method_call(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    fn run(src: &str) -> Vec<Finding> {
        TryIntoUnwrapCheck.run(&parse_file(src).unwrap(), src)
    }

    const PRELUDE: &str = r#"
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct C;

#[contractimpl]
impl C {
"#;

    #[test]
    fn flags_try_into_unwrap() {
        let src = format!(
            "{PRELUDE}    pub fn bad(env: Env, val: u64) -> u32 {{ val.try_into().unwrap() }}\n}}"
        );
        let hits = run(&src);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::Low);
        assert_eq!(hits[0].check_name, CHECK_NAME);
        assert!(hits[0].description.contains("try_into"));
    }

    #[test]
    fn does_not_flag_unwrap_without_try_into() {
        let src = format!(
            "{PRELUDE}    pub fn good(env: Env) -> Option<u32> {{ Some(42).into() }}\n}}"
        );
        let hits = run(&src);
        assert!(hits.is_empty());
    }

    #[test]
    fn flags_multiple_try_into_unwraps() {
        let src = format!(
            "{PRELUDE}    pub fn bad(env: Env, a: u64, b: u64) -> (u32, u32) {{\n                (\n                    a.try_into().unwrap(),\n                    b.try_into().unwrap(),\n                )\n            }}\n}}"
        );
        let hits = run(&src);
        assert_eq!(hits.len(), 2);
    }

    #[test]
    fn does_not_flag_try_into_ok() {
        let src = format!(
            "{PRELUDE}    pub fn good(env: Env, val: u64) -> Result<u32, _> {{ val.try_into() }}\n}}"
        );
        let hits = run(&src);
        assert!(hits.is_empty());
    }

    #[test]
    fn flags_try_into_expect() {
        let src = format!(
            "{PRELUDE}    pub fn bad(env: Env, val: u64) -> u32 {{ val.try_into().expect(\"msg\") }}\n}}"
        );
        let hits = run(&src);
        // .expect() is not unwrap, so this should not flag (expect is also dangerous but different check)
        assert!(hits.is_empty());
    }

    #[test]
    fn does_not_flag_try_into_unwrap_or() {
        let src = format!(
            "{PRELUDE}    pub fn good(env: Env, val: u64) -> u32 {{ val.try_into().unwrap_or(0) }}\n}}"
        );
        let hits = run(&src);
        assert!(hits.is_empty());
    }

    #[test]
    fn does_not_flag_try_into_unwrap_or_else() {
        let src = format!(
            "{PRELUDE}    pub fn good(env: Env, val: u64) -> u32 {{ val.try_into().unwrap_or_else(|_| 0) }}\n}}"
        );
        let hits = run(&src);
        assert!(hits.is_empty());
    }
}
