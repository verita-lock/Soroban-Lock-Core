//! Temporary storage used for long-lived contract state (admin, owner, total_supply, etc.).

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, Stmt};

const CHECK_NAME: &str = "temp-for-persistent-data";

pub struct TempForPersistentDataCheck;

impl Check for TempForPersistentDataCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let mut v = TempPersistentVisitor {
                fn_name: method.sig.ident.to_string(),
                symbol_vars: HashMap::new(),
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

fn first_arg_str(m: &ExprMethodCall, symbol_vars: &HashMap<String, String>) -> Option<String> {
    let arg = m.args.first()?;
    Some(match arg {
        Expr::Reference(r) => expr_to_string(&r.expr, symbol_vars),
        other => expr_to_string(other, symbol_vars),
    })
}

fn expr_to_string(expr: &Expr, symbol_vars: &HashMap<String, String>) -> String {
    match expr {
        Expr::Path(p) => {
            let name = p
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();
            symbol_vars.get(&name).cloned().unwrap_or(name)
        }
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

fn pat_ident_name(pat: &syn::Pat) -> Option<String> {
    match pat {
        syn::Pat::Ident(pat_ident) => Some(pat_ident.ident.to_string()),
        syn::Pat::Type(pat_type) => pat_ident_name(&pat_type.pat),
        _ => None,
    }
}

/// Extract the literal string from a `Symbol::new(&env, "literal")` call.
fn symbol_new_literal(expr: &Expr) -> Option<String> {
    if let Expr::Call(call) = expr {
        if let Expr::Path(p) = &*call.func {
            let is_symbol_new = p.path.segments.len() >= 2
                && p.path.segments[p.path.segments.len() - 2].ident == "Symbol"
                && p.path.segments.last()?.ident == "new";
            if is_symbol_new {
                for arg in &call.args {
                    if let Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = arg
                    {
                        return Some(s.value());
                    }
                }
            }
        }
    }
    None
}

fn key_looks_like_persistent_data(key: &str) -> bool {
    let lower = key.to_lowercase();
    lower.contains("admin")
        || lower.contains("owner")
        || lower.contains("total_supply")
        || lower.contains("balance_of")
        || lower.contains("allowance")
        || lower.contains("config")
        || lower.contains("fee")
        || lower.contains("rate")
}

struct TempPersistentVisitor<'a> {
    fn_name: String,
    symbol_vars: HashMap<String, String>,
    out: &'a mut Vec<Finding>,
}

impl Visit<'_> for TempPersistentVisitor<'_> {
    fn visit_stmt(&mut self, i: &Stmt) {
        if let Stmt::Local(local) = i {
            if let Some(init) = &local.init {
                if let Some(literal) = symbol_new_literal(&init.expr) {
                    if let Some(name) = pat_ident_name(&local.pat) {
                        self.symbol_vars.insert(name, literal);
                    }
                }
            }
        }
        visit::visit_stmt(self, i);
    }

    fn visit_expr_method_call(&mut self, i: &ExprMethodCall) {
        if i.method == "set" && receiver_chain_contains_temporary(&i.receiver) {
            if let Some(key) = first_arg_str(i, &self.symbol_vars) {
                if key_looks_like_persistent_data(&key) {
                    self.out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::High,
                        file_path: String::new(),
                        line: i.span().start().line,
                        function_name: self.fn_name.clone(),
                        description: format!(
                            "Method `{}` stores a persistent data key (`{}`) in \
                             `env.storage().temporary()`. Temporary storage expires with TTL, \
                             causing permanent data loss. Use `persistent()` or `instance()` instead.",
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
    fn flags_admin_in_temporary() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
const ADMIN: soroban_sdk::Symbol = symbol_short!("admin");
#[contractimpl]
impl C {
    pub fn set_admin(env: Env, new_admin: Address) {
        env.storage().temporary().set(&ADMIN, &new_admin);
    }
}
"#,
        )?;
        let hits = TempForPersistentDataCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::High);
        assert_eq!(hits[0].check_name, CHECK_NAME);
        Ok(())
    }

    #[test]
    fn flags_total_supply_in_temporary() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Address, Env, Symbol};
pub struct C;
#[contractimpl]
impl C {
    pub fn init(env: Env, supply: i128) {
        let key = Symbol::new(&env, "total_supply");
        env.storage().temporary().set(&key, &supply);
    }
}
"#,
        )?;
        let hits = TempForPersistentDataCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        Ok(())
    }

    #[test]
    fn no_finding_for_persistent_admin() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
const ADMIN: soroban_sdk::Symbol = symbol_short!("admin");
#[contractimpl]
impl C {
    pub fn set_admin(env: Env, new_admin: Address) {
        env.storage().persistent().set(&ADMIN, &new_admin);
    }
}
"#,
        )?;
        let hits = TempForPersistentDataCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn no_finding_for_non_persistent_temp_key() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, symbol_short, Env};
pub struct C;
const COUNTER: soroban_sdk::Symbol = symbol_short!("cnt");
#[contractimpl]
impl C {
    pub fn tick(env: Env) {
        env.storage().temporary().set(&COUNTER, &1u32);
    }
}
"#,
        )?;
        let hits = TempForPersistentDataCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }
}
