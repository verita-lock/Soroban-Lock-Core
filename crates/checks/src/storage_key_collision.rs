//! Flags potential storage key collisions where different keys have similar names that could lead to accidental overwrites.
//!
//! Storage keys should be unique and descriptive. Similar key names (e.g., "owner", "owner_addr", "owner_address")
//! can lead to accidental overwrites if developers use the wrong key in different contexts.

use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, Lit, LitStr, Pat, Stmt};

const CHECK_NAME: &str = "storage-key-collision";

/// Flags storage keys that have similar names and may cause accidental overwrites.
/// Detects patterns like "owner", "owner_addr", "owner_address" in the same contract.
pub struct StorageKeyCollisionCheck;

impl Check for StorageKeyCollisionCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        
        // Collect all storage keys used in the contract
        let mut keys = Vec::new();
        
        // Look for storage set calls with string literal keys
        for item in &file.items {
            match item {
                syn::Item::Fn(func) => {
                    let mut v = KeyVisitor {
                        keys: &mut keys,
                    };
                    v.visit_item_fn(func);
                }
                _ => {}
            }
        }
        
        // Check for similar keys
        for i in 0..keys.len() {
            for j in (i + 1)..keys.len() {
                let key1 = &keys[i].0;
                let key2 = &keys[j].0;
                
                // Check for similarity: same prefix or suffix, or one is substring of another
                if key1.len() >= 3 && key2.len() >= 3 {
                    if key1.starts_with(key2) || key2.starts_with(key1) ||
                       key1.contains(key2) || key2.contains(key1) ||
                       (key1.len() > key2.len() && key1[..key2.len()].eq_ignore_ascii_case(key2)) ||
                       (key2.len() > key1.len() && key2[..key1.len()].eq_ignore_ascii_case(key1)) {
                        
                        // Skip obvious cases like "owner" and "owner_addr"
                        if !is_obvious_prefix_suffix(key1, key2) {
                            out.push(Finding {
                                check_name: CHECK_NAME.to_string(),
                                severity: Severity::Medium,
                                file_path: String::new(),
                                line: keys[i].1,
                                function_name: String::new(),
                                description: format!("Potential storage key collision between '{}' and '{}'. Consider using more distinct key names to avoid accidental overwrites.", key1, key2),
                            });
                        }
                    }
                }
            }
        }
        
        out
    }
}

fn is_obvious_prefix_suffix(key1: &str, key2: &str) -> bool {
    // Check for common patterns like "owner" and "owner_addr"
    let key1_lower = key1.to_lowercase();
    let key2_lower = key2.to_lowercase();
    
    if key1_lower == key2_lower {
        return true;
    }
    
    // Check if one is a prefix of the other with common suffixes
    if key1_lower.starts_with(&key2_lower) || key2_lower.starts_with(&key1_lower) {
        let shorter = if key1_lower.len() < key2_lower.len() { &key1_lower } else { &key2_lower };
        let longer = if key1_lower.len() >= key2_lower.len() { &key1_lower } else { &key2_lower };
        
        // Check for common suffixes
        let suffixes = ["_addr", "_address", "_id", "_identifier", "_key", "_hash"];
        for suffix in &suffixes {
            if longer.ends_with(suffix) && longer[..longer.len() - suffix.len()] == *shorter {
                return true;
            }
        }
    }
    
    false
}

struct KeyVisitor<'a> {
    keys: &'a mut Vec<(String, usize)>,
}

impl<'a> Visit<'a> for KeyVisitor<'a> {
    fn visit_expr_method_call(&mut self, i: &'a ExprMethodCall) {
        // Look for storage set calls: env.storage().persistent().set(&"key", &val)
        if i.method == "set" {
            // Check if the first argument is a string literal
            if let Some(arg) = i.args.first() {
                if let syn::Expr::Reference(ref_ref) = arg {
                    if let syn::Expr::Lit(lit) = &*ref_ref.expr {
                        if let syn::Lit::Str(s) = &lit.lit {
                            self.keys.push((s.value(), lit.span().start().line));
                        }
                    }
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
        Ok(StorageKeyCollisionCheck.run(&file, src))
    }

    #[test]
    fn flags_similar_keys() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};

pub struct C;

const OWNER: soroban_sdk::Symbol = symbol_short!("owner");
const OWNER_ADDR: soroban_sdk::Symbol = symbol_short!("owner_addr");

#[contractimpl]
impl C {
    pub fn store_owner(env: Env, owner: soroban_sdk::Address) {
        env.storage().persistent().set(&OWNER, &owner);
        env.storage().persistent().set(&OWNER_ADDR, &owner);
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
    fn passes_when_keys_are_distinct() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};

pub struct C;

const OWNER: soroban_sdk::Symbol = symbol_short!("owner");
const BALANCE: soroban_sdk::Symbol = symbol_short!("balance");

#[contractimpl]
impl C {
    pub fn store_data(env: Env, owner: soroban_sdk::Address, balance: i128) {
        env.storage().persistent().set(&OWNER, &owner);
        env.storage().persistent().set(&BALANCE, &balance);
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }
}
