//! Admin address stored in contract storage but never used in a `require_auth`
//! call — the stored admin is cosmetic and provides no actual access control.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File};

const CHECK_NAME: &str = "admin-stored-unused";

pub struct AdminStoredUnusedCheck;

impl Check for AdminStoredUnusedCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut stores_admin = false;
        let mut has_admin_require_auth = false;

        for method in contractimpl_functions(file) {
            let mut scan = AdminUsageScan::default();
            scan.visit_block(&method.block);
            if scan.stores_admin {
                stores_admin = true;
            }
            if scan.admin_require_auth {
                has_admin_require_auth = true;
            }
        }

        if stores_admin && !has_admin_require_auth {
            // Find the first function that stores admin to report the line.
            for method in contractimpl_functions(file) {
                let mut scan = AdminUsageScan::default();
                scan.visit_block(&method.block);
                if scan.stores_admin {
                    let fn_name = method.sig.ident.to_string();
                    let line = method.sig.fn_token.span().start().line;
                    return vec![Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::High,
                        file_path: String::new(),
                        line,
                        function_name: fn_name.clone(),
                        description: format!(
                            "Function `{fn_name}` stores an admin/owner address but no \
                             function in this contract calls `require_auth` on the stored \
                             admin. The stored admin has no actual authority."
                        ),
                    }];
                }
            }
        }
        vec![]
    }
}

/// Detects `.set(admin_key, ...)` writes and `.require_auth()` calls on
/// variables whose name contains "admin" or "owner".
#[derive(Default)]
struct AdminUsageScan {
    stores_admin: bool,
    admin_require_auth: bool,
}

fn is_admin_key(expr: &Expr) -> bool {
    match expr {
        Expr::Reference(r) => is_admin_key(&r.expr),
        Expr::Path(p) => {
            let s = p
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::")
                .to_lowercase();
            s.contains("admin") || s.contains("owner")
        }
        Expr::Macro(m) => {
            let s = m
                .mac
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::")
                .to_lowercase();
            // symbol_short!("admin") etc.
            s.contains("symbol") || {
                m.mac
                    .tokens
                    .to_string()
                    .to_lowercase()
                    .contains("admin")
                    || m.mac.tokens.to_string().to_lowercase().contains("owner")
            }
        }
        Expr::Lit(l) => {
            if let syn::Lit::Str(s) = &l.lit {
                let v = s.value().to_lowercase();
                v.contains("admin") || v.contains("owner")
            } else {
                false
            }
        }
        _ => false,
    }
}

fn receiver_ident(expr: &Expr) -> String {
    match expr {
        Expr::Path(p) => p
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::"),
        Expr::MethodCall(m) => receiver_ident(&m.receiver),
        Expr::Reference(r) => receiver_ident(&r.expr),
        _ => String::new(),
    }
}

impl<'ast> Visit<'ast> for AdminUsageScan {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        let method = i.method.to_string();

        // Detect storage().*.set(&admin_key, value)
        if method == "set" && i.args.len() >= 2 {
            if let Some(key_arg) = i.args.first() {
                if is_admin_key(key_arg) {
                    self.stores_admin = true;
                }
            }
        }

        // Detect admin_var.require_auth() where var name contains admin/owner
        if method == "require_auth" || method == "require_auth_for_args" {
            let recv = receiver_ident(&i.receiver).to_lowercase();
            if recv.contains("admin") || recv.contains("owner") {
                self.admin_require_auth = true;
            }
        }

        visit::visit_expr_method_call(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    #[test]
    fn flags_admin_stored_but_never_used_in_auth() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }
    pub fn do_thing(env: Env) {
        env.require_auth();
    }
}
"#,
        )?;
        let hits = AdminStoredUnusedCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::High);
        Ok(())
    }

    #[test]
    fn passes_when_admin_used_in_require_auth() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }
    pub fn protected(env: Env) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        admin.require_auth();
    }
}
"#,
        )?;
        let hits = AdminStoredUnusedCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn ignores_contract_without_admin_storage() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn set_value(env: Env, v: u32) {
        env.storage().instance().set(&symbol_short!("val"), &v);
    }
}
"#,
        )?;
        let hits = AdminStoredUnusedCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }
}
