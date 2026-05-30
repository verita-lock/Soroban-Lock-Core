//! Flags inconsistent use of storage types across different versions or incompatible types.
//!
//! Storage types should be consistent across the contract. Mixing different versions
//! or incompatible types can lead to unexpected behavior and security vulnerabilities.

use crate::{Check, Finding, Severity};
use quote::ToTokens;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, Lit, LitStr, Pat, Stmt};

const CHECK_NAME: &str = "storage-type-version";

/// Flags inconsistent use of storage types in the same contract.
/// Detects mixing of different storage types (e.g., persistent vs instance) or versions.
pub struct StorageTypeVersionCheck;

impl Check for StorageTypeVersionCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        
        // Collect all storage types used in the contract
        let mut storage_types = std::collections::HashSet::new();
        
        for item in &file.items {
            match item {
                syn::Item::Fn(func) => {
                    let mut v = StorageTypeVisitor {
                        storage_types: &mut storage_types,
                    };
                    v.visit_item_fn(func);
                }
                _ => {}
            }
        }
        
        // Check for inconsistent storage types
        if storage_types.len() > 1 {
            out.push(Finding {
                check_name: CHECK_NAME.to_string(),
                severity: Severity::Medium,
                file_path: String::new(),
                line: 0,
                function_name: String::new(),
                description: format!("Contract uses multiple storage types: {:?}. Consider using a consistent storage tier for better predictability and security.", storage_types),
            });
        }
        Expr::Field(f) => receiver_chain_contains_storage(&f.base),
        _ => false,
    }
}

fn is_storage_set_call(m: &ExprMethodCall) -> bool {
    m.method == "set" && receiver_chain_contains_storage(&m.receiver)
}

fn is_storage_get_call(m: &ExprMethodCall) -> bool {
    m.method == "get" && receiver_chain_contains_storage(&m.receiver)
}

fn extract_key_from_call(m: &ExprMethodCall) -> Option<String> {
    if m.args.is_empty() {
        return None;
    }
    Some(m.args[0].to_token_stream().to_string())
}

fn extract_value_type_from_set(m: &ExprMethodCall) -> Option<String> {
    if m.args.len() < 2 {
        return None;
    }
    Some(m.args[1].to_token_stream().to_string())
}

struct StorageTypeVisitor<'a> {
    storage_types: &'a mut std::collections::HashSet<String>,
}

impl<'a> Visit<'a> for StorageTypeVisitor<'a> {
    fn visit_expr_method_call(&mut self, i: &'a ExprMethodCall) {
        // Look for storage method calls
        if i.method == "persistent" || i.method == "instance" || i.method == "temporary" {
            // Check if this is part of a storage chain
            if let Expr::MethodCall(mc) = &*i.receiver {
                if mc.method == "storage" {
                    self.storage_types.insert(i.method.to_string());
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
        Ok(StorageTypeVersionCheck.run(&file, src))
    }

    #[test]
    fn flags_mixed_storage_types() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn mixed_storage(env: Env) {
        // Uses both persistent and instance storage
        env.storage().persistent().set(&KEY, &1);
        env.storage().instance().set(&KEY, &2);
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
    fn passes_when_single_storage_type() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn single_storage(env: Env) {
        // Only uses persistent storage
        env.storage().persistent().set(&KEY, &1);
        env.storage().persistent().set(&KEY2, &2);
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }
}
