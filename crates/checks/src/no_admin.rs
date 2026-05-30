//! Detects contracts that write storage but expose no admin ownership mechanism.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File};

const CHECK_NAME: &str = "no-admin";

/// A contract writes storage but has no admin-like storage key or admin ownership
/// function.
pub struct NoAdminCheck;

impl Check for NoAdminCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut scan = NoAdminScan::default();

        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            if fn_name == "set_admin" || fn_name == "transfer_ownership" {
                scan.has_admin_fn = true;
            }

            let mut method_scan = StorageScan::default();
            method_scan.visit_block(&method.block);
            if method_scan.has_storage_set && scan.first_storage_fn_name.is_none() {
                scan.first_storage_fn_name = Some(fn_name.clone());
                scan.first_storage_fn_line = Some(method.sig.fn_token.span().start().line);
            }
            scan.has_storage_set |= method_scan.has_storage_set;
            scan.has_admin_key |= method_scan.has_admin_key;
        }

        if scan.has_storage_set && !scan.has_admin_key && !scan.has_admin_fn {
            let line = scan.first_storage_fn_line.unwrap_or(1);
            let function_name = scan.first_storage_fn_name.unwrap_or_else(|| "<unknown>".to_string());
            return vec![Finding {
                check_name: CHECK_NAME.to_string(),
                severity: Severity::Medium,
                file_path: String::new(),
                line,
                function_name,
                description: format!(
                    "Contract writes to storage but has no admin/owner/operator storage key "
                    "and no ownership function named `set_admin` or `transfer_ownership`. "
                    "This contract may be ungovernable."
                ),
            }];
        }

        Vec::new()
    }
}

#[derive(Default)]
struct NoAdminScan {
    has_storage_set: bool,
    has_admin_key: bool,
    has_admin_fn: bool,
    first_storage_fn_name: Option<String>,
    first_storage_fn_line: Option<usize>,
}

#[derive(Default)]
struct StorageScan {
    has_storage_set: bool,
    has_admin_key: bool,
}

fn key_expr_text(expr: &Expr) -> String {
    match expr {
        Expr::Reference(r) => key_expr_text(&r.expr),
        Expr::Path(p) => p
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::"),
        Expr::Lit(l) => match &l.lit {
            syn::Lit::Str(s) => s.value(),
            _ => String::new(),
        },
        _ => String::new(),
    }
}

fn is_admin_key(text: &str) -> bool {
    let t = text.to_lowercase();
    t.contains("admin") || t.contains("owner") || t.contains("operator")
}

fn receiver_chain_contains(expr: &Expr, method: &str) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            if m.method == method {
                return true;
            }
            receiver_chain_contains(&m.receiver, method)
        }
        Expr::Field(f) => receiver_chain_contains(&f.base, method),
        _ => false,
    }
}

fn is_storage_call(m: &ExprMethodCall) -> bool {
    receiver_chain_contains(&m.receiver, "storage")
}

impl<'ast> Visit<'ast> for StorageScan {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_storage_call(i) {
            if let Some(key_arg) = i.args.first() {
                let key_text = key_expr_text(key_arg);
                if is_admin_key(&key_text) {
                    self.has_admin_key = true;
                }
            }
            if i.method == "set" {
                self.has_storage_set = true;
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
    fn flags_storage_writes_without_admin_mechanism() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn update_data(env: Env, value: i32) {
        env.storage().instance().set(&"data", &value);
    }
}
"#,
        )?;
        let hits = NoAdminCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::Medium);
        assert_eq!(hits[0].function_name, "update_data");
        Ok(())
    }

    #[test]
    fn passes_when_admin_storage_key_exists() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Address, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&"admin", &admin);
    }

    pub fn update_data(env: Env, value: i32) {
        env.storage().instance().set(&"data", &value);
    }
}
"#,
        )?;
        let hits = NoAdminCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn passes_when_set_admin_function_exists() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Address, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn set_admin(env: Env, admin: Address) {
        env.storage().instance().set(&"admin", &admin);
    }

    pub fn update_data(env: Env, value: i32) {
        env.storage().instance().set(&"data", &value);
    }
}
"#,
        )?;
        let hits = NoAdminCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }
}
