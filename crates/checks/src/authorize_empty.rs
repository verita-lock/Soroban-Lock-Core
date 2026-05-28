//! Calling authorize_as_current_contract with empty invocation vector.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprArray, ExprMethodCall, File};

const CHECK_NAME: &str = "authorize-empty";

/// Detects `env.authorize_as_current_contract(&[])` or similar calls with empty invocation vectors.
pub struct AuthorizeEmptyCheck;

impl Check for AuthorizeEmptyCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut scan = AuthorizeEmptyScan {
                fn_name: fn_name.clone(),
                out: &mut out,
            };
            scan.visit_block(&method.block);
        }
        out
    }
}

fn is_authorize_as_current_contract_call(m: &ExprMethodCall) -> bool {
    m.method == "authorize_as_current_contract"
        && matches!(&*m.receiver, Expr::Path(p) if p.path.is_ident("env"))
}

fn has_empty_array_arg(m: &ExprMethodCall) -> bool {
    m.args.len() == 1
        && matches!(&m.args[0], Expr::Array(ExprArray { elems, .. }) if elems.is_empty())
}

struct AuthorizeEmptyScan<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for AuthorizeEmptyScan<'_> {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_authorize_as_current_contract_call(i) && has_empty_array_arg(i) {
            self.out.push(Finding {
                check_name: CHECK_NAME.to_string(),
                severity: Severity::High,
                file_path: String::new(),
                line: i.span().start().line,
                function_name: self.fn_name.clone(),
                description: format!(
                    "Function `{}` calls `env.authorize_as_current_contract(&[])` with an empty invocation vector. \
                     This authorizes nothing but still consumes compute. Likely a bug — specify the intended sub-contract calls.",
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
    use syn::parse_file;

    #[test]
    fn detects_authorize_with_empty_array() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn vulnerable(env: Env) {
        env.authorize_as_current_contract(&[]);
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = AuthorizeEmptyCheck;
        let findings = check.run(&file, code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].check_name, CHECK_NAME);
        assert_eq!(findings[0].severity, Severity::High);
    }

    #[test]
    fn allows_authorize_with_non_empty_array() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn safe(env: Env) {
        env.authorize_as_current_contract(&[some_invocation]);
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = AuthorizeEmptyCheck;
        let findings = check.run(&file, code);
        assert!(findings.is_empty());
    }

    #[test]
    fn ignores_other_method_calls() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn safe(env: Env) {
        env.some_other_method(&[]);
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = AuthorizeEmptyCheck;
        let findings = check.run(&file, code);
        assert!(findings.is_empty());
    }
}
