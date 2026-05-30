//! Flags storage value type version mismatch in get/set across functions.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use quote::ToTokens;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File};

const CHECK_NAME: &str = "storage-type-version";

/// Detects inconsistent types used with the same storage key across functions.
pub struct StorageTypeVersionCheck;

impl Check for StorageTypeVersionCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        let mut key_types: HashMap<String, (String, usize, String)> = HashMap::new();

        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut v = TypeCollector {
                fn_name: fn_name.clone(),
                key_types: &mut key_types,
            };
            v.visit_block(&method.block);
        }

        // Check for type mismatches
        for (key, types) in key_types.iter() {
            let mut type_set = std::collections::HashSet::new();
            type_set.insert(types.0.clone());

            for (other_key, other_types) in key_types.iter() {
                if key == other_key && types.0 != other_types.0 {
                    out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::Medium,
                        file_path: String::new(),
                        line: types.1,
                        function_name: types.2.clone(),
                        description: format!(
                            "Storage key {:?} has inconsistent types: {} vs {}. \
                             Deserialization will fail or produce garbled results.",
                            key, types.0, other_types.0
                        ),
                    });
                    break;
                }
            }
        }

        out
    }
}

fn receiver_chain_contains_storage(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            if m.method == "storage" {
                return true;
            }
            receiver_chain_contains_storage(&m.receiver)
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

struct TypeCollector<'a> {
    fn_name: String,
    key_types: &'a mut HashMap<String, (String, usize, String)>,
}

impl<'a> Visit<'a> for TypeCollector<'a> {
    fn visit_expr_method_call(&mut self, i: &'a ExprMethodCall) {
        if is_storage_set_call(i) {
            if let Some(key) = extract_key_from_call(i) {
                if let Some(val_type) = extract_value_type_from_set(i) {
                    self.key_types.insert(
                        key,
                        (val_type, i.span().start().line, self.fn_name.clone()),
                    );
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
    fn flags_type_mismatch() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn store_u32(env: Env) {
        env.storage().instance().set(&symbol_short!("data"), &42u32);
    }

    pub fn store_string(env: Env) {
        env.storage().instance().set(&symbol_short!("data"), &"text");
    }
}
"#,
        )?;
        assert!(hits.len() > 0);
        assert_eq!(hits[0].severity, Severity::Medium);
        Ok(())
    }

    #[test]
    fn passes_consistent_types() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn store_first(env: Env) {
        env.storage().instance().set(&symbol_short!("data"), &42u32);
    }

    pub fn store_second(env: Env) {
        env.storage().instance().set(&symbol_short!("data"), &100u32);
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }
}
