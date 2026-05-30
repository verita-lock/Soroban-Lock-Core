//! Admin address read from storage and compared with `==` instead of calling
//! `require_auth` — bypasses host-level signature verification.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprBinary, ExprMethodCall, File};

const CHECK_NAME: &str = "admin-eq-instead-of-auth";

pub struct AdminEqInsteadOfAuthCheck;

impl Check for AdminEqInsteadOfAuthCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut scan = AdminEqScan {
                fn_name: fn_name.clone(),
                out: &mut out,
                has_require_auth: false,
                admin_locals: Vec::new(),
            };
            scan.visit_block(&method.block);
        }
        out
    }
}

struct AdminEqScan<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
    has_require_auth: bool,
    /// Local variable names that were assigned from an admin/owner storage read.
    admin_locals: Vec<String>,
}

fn is_admin_storage_get(expr: &Expr) -> bool {
    // Matches: env.storage().*.get(&admin_key)
    match expr {
        Expr::MethodCall(m) if m.method == "get" || m.method == "get_unchecked" => {
            if let Some(key) = m.args.first() {
                return is_admin_key_expr(key);
            }
            false
        }
        Expr::MethodCall(m) if m.method == "unwrap" || m.method == "unwrap_or_default" || m.method == "unwrap_or" => {
            is_admin_storage_get(&m.receiver)
        }
        _ => false,
    }
}

fn is_admin_key_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Reference(r) => is_admin_key_expr(&r.expr),
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
            let tokens = m.mac.tokens.to_string().to_lowercase();
            tokens.contains("admin") || tokens.contains("owner")
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

fn expr_ident(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(p) if p.path.segments.len() == 1 => {
            Some(p.path.segments[0].ident.to_string())
        }
        _ => None,
    }
}

impl<'ast> Visit<'ast> for AdminEqScan<'_> {
    fn visit_local(&mut self, i: &'ast syn::Local) {
        // let admin = env.storage()...get(&admin_key).unwrap();
        if let Some(init) = &i.init {
            if is_admin_storage_get(&init.expr) {
                if let syn::Pat::Ident(p) = &i.pat {
                    self.admin_locals.push(p.ident.to_string());
                }
            }
        }
        visit::visit_local(self, i);
    }

    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if i.method == "require_auth" || i.method == "require_auth_for_args" {
            self.has_require_auth = true;
        }
        visit::visit_expr_method_call(self, i);
    }

    fn visit_expr_binary(&mut self, i: &'ast ExprBinary) {
        if !self.has_require_auth
            && matches!(i.op, syn::BinOp::Eq(_) | syn::BinOp::Ne(_))
        {
            let left_id = expr_ident(&i.left);
            let right_id = expr_ident(&i.right);
            let involves_admin = [&left_id, &right_id].iter().any(|id| {
                if let Some(name) = id {
                    let lower = name.to_lowercase();
                    lower.contains("admin")
                        || lower.contains("owner")
                        || self.admin_locals.contains(name)
                } else {
                    false
                }
            });
            if involves_admin {
                self.out.push(Finding {
                    check_name: CHECK_NAME.to_string(),
                    severity: Severity::High,
                    file_path: String::new(),
                    line: i.span().start().line,
                    function_name: self.fn_name.clone(),
                    description: format!(
                        "Function `{}` compares an admin/owner address with `==` instead of \
                         calling `require_auth()`. This bypasses host-level signature \
                         verification and can be spoofed.",
                        self.fn_name
                    ),
                });
            }
        }
        visit::visit_expr_binary(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    #[test]
    fn flags_admin_eq_comparison() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn protected(env: Env, caller: Address) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        if caller == admin {
            // do privileged thing
        }
    }
}
"#,
        )?;
        let hits = AdminEqInsteadOfAuthCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::High);
        assert_eq!(hits[0].function_name, "protected");
        Ok(())
    }

    #[test]
    fn passes_when_require_auth_called() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn protected(env: Env) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        admin.require_auth();
    }
}
"#,
        )?;
        let hits = AdminEqInsteadOfAuthCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn ignores_non_admin_comparison() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn check(env: Env, amount: u32) {
        if amount == 0 {
            panic!("zero");
        }
    }
}
"#,
        )?;
        let hits = AdminEqInsteadOfAuthCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }
}
