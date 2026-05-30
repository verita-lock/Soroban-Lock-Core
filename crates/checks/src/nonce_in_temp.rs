//! Nonce/sequence stored in temporary storage — expires with TTL, enabling replay attacks.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File};

const CHECK_NAME: &str = "nonce-in-temp";

pub struct NonceInTempCheck;

impl Check for NonceInTempCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let mut v = NonceInTempVisitor {
                fn_name: method.sig.ident.to_string(),
                out: &mut out,
            };
            v.visit_block(&method.block);
        }
        out
    }
}

fn receiver_chain_contains_temporary(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            m.method == "temporary" || receiver_chain_contains_temporary(&m.receiver)
        }
        Expr::Field(f) => receiver_chain_contains_temporary(&f.base),
        _ => false,
    }
}

fn first_arg_str(m: &ExprMethodCall) -> Option<String> {
    let arg = m.args.first()?;
    Some(match arg {
        Expr::Reference(r) => expr_to_string(&r.expr),
        other => expr_to_string(other),
    })
}

fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Path(p) => p
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default(),
        Expr::Lit(l) => match &l.lit {
            syn::Lit::Str(s) => s.value(),
            _ => String::new(),
        },
        Expr::Macro(m) => m
            .mac
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

fn key_looks_like_nonce(key: &str) -> bool {
    let lower = key.to_lowercase();
    lower.contains("nonce")
        || lower.contains("seqno")
        || lower.contains("counter")
        || lower.contains("sequence")
}

struct NonceInTempVisitor<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
}

impl Visit<'_> for NonceInTempVisitor<'_> {
    fn visit_expr_method_call(&mut self, i: &ExprMethodCall) {
        if i.method == "set" && receiver_chain_contains_temporary(&i.receiver) {
            if let Some(key) = first_arg_str(i) {
                if key_looks_like_nonce(&key) {
                    self.out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::High,
                        file_path: String::new(),
                        line: i.span().start().line,
                        function_name: self.fn_name.clone(),
                        description: format!(
                            "Method `{}` stores a replay-protection key (`{}`) in \
                             `env.storage().temporary()`. Temporary storage expires with TTL, \
                             allowing replay of old transactions once the nonce is gone. Use \
                             `persistent()` storage instead.",
                            self.fn_name, key
                        ),
                    });
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

    #[test]
    fn flags_nonce_key_in_temporary() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};
pub struct C;
const NONCE: soroban_sdk::Symbol = symbol_short!("nonce");
#[contractimpl]
impl C {
    pub fn execute(env: Env, n: u64) {
        env.storage().temporary().set(&NONCE, &n);
    }
}
"#,
        )?;
        let hits = NonceInTempCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::High);
        assert!(hits[0].description.contains("NONCE"));
        Ok(())
    }

    #[test]
    fn flags_sequence_key_in_temporary() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};
pub struct C;
const SEQ: soroban_sdk::Symbol = symbol_short!("sequence");
#[contractimpl]
impl C {
    pub fn submit(env: Env, s: u64) {
        env.storage().temporary().set(&SEQ, &s);
    }
}
"#,
        )?;
        let hits = NonceInTempCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        Ok(())
    }

    #[test]
    fn flags_counter_key_in_temporary() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};
pub struct C;
const COUNTER: soroban_sdk::Symbol = symbol_short!("counter");
#[contractimpl]
impl C {
    pub fn inc(env: Env) {
        env.storage().temporary().set(&COUNTER, &1u64);
    }
}
"#,
        )?;
        let hits = NonceInTempCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        Ok(())
    }

    #[test]
    fn no_finding_for_persistent_nonce() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};
pub struct C;
const NONCE: soroban_sdk::Symbol = symbol_short!("nonce");
#[contractimpl]
impl C {
    pub fn execute(env: Env, n: u64) {
        env.storage().persistent().set(&NONCE, &n);
    }
}
"#,
        )?;
        let hits = NonceInTempCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn no_finding_for_unrelated_temp_key() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};
pub struct C;
const CACHE: soroban_sdk::Symbol = symbol_short!("cache");
#[contractimpl]
impl C {
    pub fn store(env: Env, v: u32) {
        env.storage().temporary().set(&CACHE, &v);
    }
}
"#,
        )?;
        let hits = NonceInTempCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }
}
