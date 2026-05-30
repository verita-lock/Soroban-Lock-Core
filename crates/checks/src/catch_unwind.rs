//! Detects `std::panic::catch_unwind` usage in Soroban contracts.
//!
//! `catch_unwind` is undefined behavior in WASM targets and will abort the transaction.
//! It should never be used in Soroban contracts.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprCall, File};

const CHECK_NAME: &str = "catch-unwind";

/// Flags `std::panic::catch_unwind`, `::std::panic::catch_unwind`, and `catch_unwind`
/// function calls inside `#[contractimpl]` function bodies.
pub struct CatchUnwindCheck;

impl Check for CatchUnwindCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut scan = CatchUnwindScan {
                fn_name,
                out: &mut out,
            };
            scan.visit_block(&method.block);
        }
        out
    }
}

struct CatchUnwindScan<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for CatchUnwindScan<'_> {
    fn visit_expr_call(&mut self, i: &'ast ExprCall) {
        if is_catch_unwind_call(i) {
            let line = i.span().start().line;
            self.out.push(Finding {
                check_name: CHECK_NAME.to_string(),
                severity: Severity::High,
                file_path: String::new(),
                line,
                function_name: self.fn_name.clone(),
                description: format!(
                    "Method `{}` uses `catch_unwind`. This is undefined behavior in WASM \
                     targets and will abort the transaction. It should never be used in \
                     Soroban contracts.",
                    self.fn_name
                ),
            });
        }
        visit::visit_expr_call(self, i);
    }
}

fn is_catch_unwind_call(call: &ExprCall) -> bool {
    let Expr::Path(p) = &*call.func else {
        return false;
    };

    let segs = &p.path.segments;

    // Match: catch_unwind
    if segs.len() == 1 && segs[0].ident == "catch_unwind" {
        return true;
    }

    // Match: std::panic::catch_unwind
    if segs.len() == 3
        && segs[0].ident == "std"
        && segs[1].ident == "panic"
        && segs[2].ident == "catch_unwind"
    {
        return true;
    }

    // Match: ::std::panic::catch_unwind
    if segs.len() == 3
        && segs[0].ident == "std"
        && segs[1].ident == "panic"
        && segs[2].ident == "catch_unwind"
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    #[test]
    fn detects_catch_unwind_direct() -> Result<(), syn::Error> {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn risky_op(env: Env) {
        let _ = catch_unwind(|| {
            // Some code
        });
    }
}
        "#;
        let file = parse_file(code)?;
        let check = CatchUnwindCheck;
        let findings = check.run(&file, code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].check_name, CHECK_NAME);
        assert_eq!(findings[0].function_name, "risky_op");
        assert_eq!(findings[0].severity, Severity::High);
        Ok(())
    }

    #[test]
    fn detects_std_panic_catch_unwind() -> Result<(), syn::Error> {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn risky_op(env: Env) {
        let result = std::panic::catch_unwind(|| {
            // Some code
        });
    }
}
        "#;
        let file = parse_file(code)?;
        let check = CatchUnwindCheck;
        let findings = check.run(&file, code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].check_name, CHECK_NAME);
        assert_eq!(findings[0].function_name, "risky_op");
        assert_eq!(findings[0].severity, Severity::High);
        Ok(())
    }

    #[test]
    fn detects_absolute_std_panic_catch_unwind() -> Result<(), syn::Error> {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn risky_op(env: Env) {
        ::std::panic::catch_unwind(|| {});
    }
}
        "#;
        let file = parse_file(code)?;
        let check = CatchUnwindCheck;
        let findings = check.run(&file, code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].check_name, CHECK_NAME);
        assert_eq!(findings[0].severity, Severity::High);
        Ok(())
    }

    #[test]
    fn ignores_outside_contractimpl() -> Result<(), syn::Error> {
        let code = r#"
pub fn regular_fn() {
    let _ = catch_unwind(|| {});
}
        "#;
        let file = parse_file(code)?;
        let check = CatchUnwindCheck;
        let findings = check.run(&file, code);
        assert_eq!(findings.len(), 0);
        Ok(())
    }

    #[test]
    fn ignores_unrelated_calls() -> Result<(), syn::Error> {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn safe_op(env: Env) {
        let _ = some_function(|| {});
    }
}
        "#;
        let file = parse_file(code)?;
        let check = CatchUnwindCheck;
        let findings = check.run(&file, code);
        assert_eq!(findings.len(), 0);
        Ok(())
    }
}
