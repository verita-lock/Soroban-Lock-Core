//! Contracts that have an `add_admin` function but no `remove_admin` or
//! `revoke_admin` function create an ever-growing admin set with no revocation
//! path.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::File;

const CHECK_NAME: &str = "admin-no-remove";

pub struct AdminNoRemoveCheck;

impl Check for AdminNoRemoveCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut add_fn: Option<(String, usize)> = None;
        let mut has_remove = false;

        for method in contractimpl_functions(file) {
            let name = method.sig.ident.to_string();
            let lower = name.to_lowercase();
            if lower.contains("add_admin") || lower.contains("add_operator") {
                if add_fn.is_none() {
                    let line = method.sig.fn_token.span().start().line;
                    add_fn = Some((name, line));
                }
            }
            if lower.contains("remove_admin")
                || lower.contains("revoke_admin")
                || lower.contains("remove_operator")
                || lower.contains("revoke_operator")
            {
                has_remove = true;
            }
        }

        if let Some((fn_name, line)) = add_fn {
            if !has_remove {
                return vec![Finding {
                    check_name: CHECK_NAME.to_string(),
                    severity: Severity::Medium,
                    file_path: String::new(),
                    line,
                    function_name: fn_name.clone(),
                    description: format!(
                        "Function `{fn_name}` adds an admin but no `remove_admin` or \
                         `revoke_admin` function exists. A compromised admin key can never \
                         be revoked."
                    ),
                }];
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    #[test]
    fn flags_add_admin_without_remove() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn add_admin(env: Env, new_admin: Address) {
        new_admin.require_auth();
    }
}
"#,
        )?;
        let hits = AdminNoRemoveCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::Medium);
        assert_eq!(hits[0].function_name, "add_admin");
        Ok(())
    }

    #[test]
    fn passes_when_remove_admin_present() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn add_admin(env: Env, new_admin: Address) {
        new_admin.require_auth();
    }
    pub fn remove_admin(env: Env, admin: Address) {
        admin.require_auth();
    }
}
"#,
        )?;
        let hits = AdminNoRemoveCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn passes_when_revoke_admin_present() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn add_admin(env: Env, new_admin: Address) {
        new_admin.require_auth();
    }
    pub fn revoke_admin(env: Env, admin: Address) {
        admin.require_auth();
    }
}
"#,
        )?;
        let hits = AdminNoRemoveCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn ignores_contract_without_add_admin() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn deposit(env: Env) {}
}
"#,
        )?;
        let hits = AdminNoRemoveCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }
}
