//! require_auth called before deployer().deploy() but deploy arguments not covered by auth.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, FnArg, Pat};

const CHECK_NAME: &str = "deploy-arg-auth";

/// Detects `deployer().deploy(wasm_hash, salt)` calls where arguments are function parameters
/// but the preceding require_auth doesn't bind to those parameters.
pub struct DeployArgAuthCheck;

impl Check for DeployArgAuthCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let param_names = extract_param_names(&method.sig.inputs);
            let mut scan = DeployArgScan {
                fn_name: fn_name.clone(),
                param_names: param_names.clone(),
                out: &mut out,
                require_auth_for_args_found: false,
            };
            scan.visit_block(&method.block);
        }
        out
    }
}

fn extract_param_names(inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>) -> Vec<String> {
    let mut names = Vec::new();
    for arg in inputs {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                names.push(pat_ident.ident.to_string());
            }
        }
    }
    names
}

fn is_require_auth_for_args_call(m: &ExprMethodCall) -> bool {
    m.method == "require_auth_for_args" && matches!(&*m.receiver, Expr::Path(p) if p.path.is_ident("env"))
}

fn is_deployer_deploy_call(m: &ExprMethodCall) -> bool {
    m.method == "deploy"
        && matches!(&*m.receiver, Expr::MethodCall(inner) if inner.method == "deployer" && matches!(&*inner.receiver, Expr::Path(p) if p.path.is_ident("env")))
}

fn arg_is_param(expr: &Expr, param_names: &[String]) -> bool {
    match expr {
        Expr::Path(p) => {
            if let Some(seg) = p.path.segments.last() {
                param_names.contains(&seg.ident.to_string())
            } else {
                false
            }
        }
        _ => false,
    }
}

struct DeployArgScan<'a> {
    fn_name: String,
    param_names: Vec<String>,
    out: &'a mut Vec<Finding>,
    require_auth_for_args_found: bool,
}

impl<'ast> Visit<'ast> for DeployArgScan<'_> {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_require_auth_for_args_call(i) {
            self.require_auth_for_args_found = true;
        }

        if is_deployer_deploy_call(i) && !self.require_auth_for_args_found {
            // Check if any deploy argument is a function parameter
            if i.args.len() >= 2 {
                let has_param_arg = i.args.iter().take(2).any(|arg| arg_is_param(arg, &self.param_names));
                if has_param_arg {
                    self.out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::High,
                        file_path: String::new(),
                        line: i.span().start().line,
                        function_name: self.fn_name.clone(),
                        description: format!(
                            "Function `{}` calls `deployer().deploy()` with user-supplied parameters but uses `require_auth()` \
                             instead of `require_auth_for_args()`. An attacker can reuse the auth to deploy with different parameters.",
                            self.fn_name
                        ),
                    });
                }
            }
        }

        visit::visit_expr_method_call(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_file;

    #[test]
    fn detects_deploy_with_param_args_and_require_auth() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn vulnerable(env: Env, wasm_hash: BytesN<32>, salt: u64) {
        env.require_auth();
        env.deployer().deploy(wasm_hash, salt);
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = DeployArgAuthCheck;
        let findings = check.run(&file, code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].check_name, CHECK_NAME);
        assert_eq!(findings[0].severity, Severity::High);
    }

    #[test]
    fn allows_deploy_with_require_auth_for_args() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn safe(env: Env, wasm_hash: BytesN<32>, salt: u64) {
        env.require_auth_for_args((wasm_hash, salt));
        env.deployer().deploy(wasm_hash, salt);
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = DeployArgAuthCheck;
        let findings = check.run(&file, code);
        assert!(findings.is_empty());
    }

    #[test]
    fn allows_deploy_with_literal_args() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn safe(env: Env) {
        env.require_auth();
        env.deployer().deploy(some_hash, 42u64);
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = DeployArgAuthCheck;
        let findings = check.run(&file, code);
        assert!(findings.is_empty());
    }
}
