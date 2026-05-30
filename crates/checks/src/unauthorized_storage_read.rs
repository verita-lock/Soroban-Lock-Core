//! Flags unauthorized reads from storage without proper authorization.
//!
//! Storage reads should be gated by `env.require_auth()` when accessing sensitive data.
//! This check detects `get` calls on storage tiers without corresponding auth calls in the same function.

use crate::util::{contractimpl_functions, is_contractimpl};
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, Lit, LitStr, Pat, Stmt};

const CHECK_NAME: &str = "unauthorized-storage-read";

/// Flags functions that contain storage `get` calls but are not properly authorized.
/// Detects `env.storage().persistent().get(...)` or `env.storage().instance().get(...)`
/// without `env.require_auth()` in the same function.
pub struct UnauthorizedStorageReadCheck;

impl Check for UnauthorizedStorageReadCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        
        // First, collect all functions that have require_auth calls
        let mut authed_functions = std::collections::HashSet::new();
        
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut v = AuthVisitor {
                has_auth: false,
            };
            v.visit_block(&method.block);
            if v.has_auth {
                authed_functions.insert(fn_name.clone());
            }
        }
        
        // Then, find all functions with storage get calls
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            if !authed_functions.contains(&fn_name) {
                let mut v = StorageGetVisitor {
                    has_storage_get: false,
                };
                v.visit_block(&method.block);
                if v.has_storage_get {
                    out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::Medium,
                        file_path: String::new(),
                        line: method.span().start().line,
                        function_name: fn_name.clone(),
                        description: format!("Function `{}` contains storage `get` calls but does not call `env.require_auth()` for authorization.", fn_name),
                    });
                }
            }
        }
        
        out
    }
}

struct AuthVisitor {
    has_auth: bool,
}

impl<'a> Visit<'a> for AuthVisitor {
    fn visit_expr_method_call(&mut self, i: &'a ExprMethodCall) {
        if i.method == "require_auth" {
            // Check if receiver is env
            if let Expr::Path(path) = &*i.receiver {
                if let Some(segment) = path.path.segments.last() {
                    if segment.ident == "env" {
                        self.has_auth = true;
                    }
                }
            }
        }
        visit::visit_expr_method_call(self, i);
    }
}

struct StorageGetVisitor {
    has_storage_get: bool,
}

impl<'a> Visit<'a> for StorageGetVisitor {
    fn visit_expr_method_call(&mut self, i: &'a ExprMethodCall) {
        // Check for storage get calls: get, has, etc.
        if i.method == "get" || i.method == "has" {
            // Check if receiver chain includes storage
            if let Expr::MethodCall(mc) = &*i.receiver {
                if mc.method == "storage" {
                    self.has_storage_get = true;
                }
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

    fn run_on_src(src: &str) -> Result<Vec<Finding>, syn::Error> {
        let file = parse_file(src)?;
        Ok(UnauthorizedStorageReadCheck.run(&file, src))
    }

    #[test]
    fn flags_unauthorized_storage_get() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn read_data(env: Env) {
        // No require_auth call
        let val = env.storage().persistent().get(&KEY);
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
    fn passes_when_require_auth_present() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn read_data(env: Env) {
        env.require_auth();
        let val = env.storage().persistent().get(&KEY);
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }
}
