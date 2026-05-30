//! Detects persistent().has() and persistent().get() called with different keys (TOCTOU).

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File};

const CHECK_NAME: &str = "storage-has-get-mismatch";

/// Flags has(key_a) followed by get(key_b) on same storage tier where keys differ.
pub struct StorageHasGetMismatchCheck;

impl Check for StorageHasGetMismatchCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut v = StorageVisitor {
                fn_name: fn_name.clone(),
                out: &mut out,
            };
            v.visit_block(&method.block);
        }
        out
    }
}

fn expr_to_string(expr: &Expr) -> String {
    expr.to_token_stream().to_string()
}

fn get_storage_tier(m: &ExprMethodCall) -> Option<String> {
    let mut current = &m.receiver;
    loop {
        match &**current {
            Expr::MethodCall(mc) => {
                if matches!(mc.method.to_string().as_str(), "persistent" | "instance" | "temporary") {
                    return Some(mc.method.to_string());
                }
                current = &mc.receiver;
            }
            _ => return None,
        }
    }
}

struct StorageVisitor<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
}

impl Visit<'_> for StorageVisitor<'_> {
    fn visit_block(&mut self, i: &syn::Block) {
        let mut has_calls: Vec<(String, String, usize)> = Vec::new();
        
        for stmt in &i.stmts {
            if let syn::Stmt::Expr(Expr::MethodCall(m), _) = stmt {
                if m.method == "has" {
                    if let Some(tier) = get_storage_tier(m) {
                        if let Some(arg) = m.args.first() {
                            let key_str = expr_to_string(arg);
                            has_calls.push((tier, key_str, m.span().start().line));
                        }
                    }
                } else if m.method == "get" {
                    if let Some(tier) = get_storage_tier(m) {
                        if let Some(arg) = m.args.first() {
                            let key_str = expr_to_string(arg);
                            for (has_tier, has_key, has_line) in &has_calls {
                                if has_tier == &tier && has_key != &key_str {
                                    self.out.push(Finding {
                                        check_name: CHECK_NAME.to_string(),
                                        severity: Severity::Medium,
                                        file_path: String::new(),
                                        line: m.span().start().line,
                                        function_name: self.fn_name.clone(),
                                        description: format!(
                                            "Mismatch in `{}` storage: has({}) at line {} but get({}) at line {}. \
                                             The has() check must use the same key as the subsequent get() call \
                                             to prevent logic errors (TOCTOU).",
                                            tier, has_key, has_line, key_str, m.span().start().line
                                        ),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        visit::visit_block(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    #[test]
    fn flags_has_get_mismatch() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};

pub struct C;

const K1: soroban_sdk::Symbol = symbol_short!("k1");
const K2: soroban_sdk::Symbol = symbol_short!("k2");

#[contractimpl]
impl C {
    pub fn process(env: Env) {
        env.require_auth();
        if env.storage().persistent().has(&K1) {
            let val = env.storage().persistent().get(&K2);
        }
    }
}
"#,
        )?;
        let hits = StorageHasGetMismatchCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::Medium);
        assert!(hits[0].description.contains("Mismatch"));
        Ok(())
    }

    #[test]
    fn passes_matching_keys() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};

pub struct C;

const K: soroban_sdk::Symbol = symbol_short!("k");

#[contractimpl]
impl C {
    pub fn process(env: Env) {
        env.require_auth();
        if env.storage().persistent().has(&K) {
            let val = env.storage().persistent().get(&K);
        }
    }
}
"#,
        )?;
        let hits = StorageHasGetMismatchCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn passes_different_tiers() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};

pub struct C;

const K1: soroban_sdk::Symbol = symbol_short!("k1");
const K2: soroban_sdk::Symbol = symbol_short!("k2");

#[contractimpl]
impl C {
    pub fn process(env: Env) {
        env.require_auth();
        if env.storage().persistent().has(&K1) {
            let val = env.storage().instance().get(&K2);
        }
    }
}
"#,
        )?;
        let hits = StorageHasGetMismatchCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }
}
