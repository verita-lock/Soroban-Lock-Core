//! require_auth called with address from temporary storage (expiration risk).

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, Ident};

const CHECK_NAME: &str = "auth-temp-storage";

/// Detects `require_auth(some_addr)` where `some_addr` was obtained from temporary() storage.
pub struct AuthTempStorageCheck;

impl Check for AuthTempStorageCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut scan = AuthTempScan {
                fn_name: fn_name.clone(),
                out: &mut out,
                temp_storage_vars: Vec::new(),
            };
            scan.visit_block(&method.block);
        }
        out
    }
}

fn is_temp_storage_get(m: &ExprMethodCall) -> bool {
    m.method == "get"
        && matches!(&*m.receiver, Expr::MethodCall(inner) if inner.method == "temporary" && matches!(&*inner.receiver, Expr::MethodCall(storage) if storage.method == "storage" && matches!(&*storage.receiver, Expr::Path(p) if p.path.is_ident("env"))))
}

fn is_require_auth_call(m: &ExprMethodCall) -> bool {
    (m.method == "require_auth" || m.method == "require_auth_for_args")
        && matches!(&*m.receiver, Expr::Path(p) if p.path.is_ident("env"))
}

fn extract_var_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(p) => {
            if let Some(seg) = p.path.segments.last() {
                Some(seg.ident.to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

struct AuthTempScan<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
    temp_storage_vars: Vec<String>,
}

impl<'ast> Visit<'ast> for AuthTempScan<'_> {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        // Track variables assigned from temporary storage
        if is_temp_storage_get(i) {
            // This is a temporary storage get call
            // We need to track if it's assigned to a variable
            // For now, we'll mark this call as a temp storage source
        }

        // Check for require_auth calls with temp storage variables
        if is_require_auth_call(i) && !i.args.is_empty() {
            if let Some(var_name) = extract_var_name(&i.args[0]) {
                if self.temp_storage_vars.contains(&var_name) {
                    self.out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::High,
                        file_path: String::new(),
                        line: i.span().start().line,
                        function_name: self.fn_name.clone(),
                        description: format!(
                            "Function `{}` calls `require_auth()` with an address from temporary storage. \
                             The address may have expired (TTL elapsed), causing auth to fail silently or authenticate against a default address.",
                            self.fn_name
                        ),
                    });
                }
            }
        }

        visit::visit_expr_method_call(self, i);
    }

    fn visit_local(&mut self, i: &'ast syn::Local) {
        // Track variable assignments from temporary storage
        if let Some(init) = &i.init {
            if let Expr::MethodCall(m) = &*init.expr {
                if is_temp_storage_get(m) {
                    if let syn::Pat::Ident(pat_ident) = &i.pat {
                        self.temp_storage_vars.push(pat_ident.ident.to_string());
                    }
                }
            }
        }
        visit::visit_local(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_file;

    #[test]
    fn detects_require_auth_with_temp_storage() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn vulnerable(env: Env) {
        let admin = env.storage().temporary().get(&Symbol::new(&env, "admin")).unwrap();
        env.require_auth(&admin);
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = AuthTempStorageCheck;
        let findings = check.run(&file, code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].check_name, CHECK_NAME);
        assert_eq!(findings[0].severity, Severity::High);
    }

    #[test]
    fn allows_require_auth_with_instance_storage() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn safe(env: Env) {
        let admin = env.storage().instance().get(&Symbol::new(&env, "admin")).unwrap();
        env.require_auth(&admin);
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = AuthTempStorageCheck;
        let findings = check.run(&file, code);
        assert!(findings.is_empty());
    }

    #[test]
    fn ignores_require_auth_without_temp_storage() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn safe(env: Env, admin: Address) {
        env.require_auth(&admin);
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = AuthTempStorageCheck;
        let findings = check.run(&file, code);
        assert!(findings.is_empty());
    }
}
