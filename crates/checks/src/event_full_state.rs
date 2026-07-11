//! Detects events publishing full storage values instead of meaningful deltas.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, Stmt};

const CHECK_NAME: &str = "event-full-state";

/// Flags `events().publish()` where data is a direct storage `get` result.
pub struct EventFullStateCheck;

impl Check for EventFullStateCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut visitor = EventPublishVisitor {
                fn_name: fn_name.clone(),
                storage_vars: HashSet::new(),
                out: &mut out,
            };
            visitor.visit_block(&method.block);
        }
        out
    }
}

struct EventPublishVisitor<'a> {
    fn_name: String,
    storage_vars: HashSet<String>,
    out: &'a mut Vec<Finding>,
}

impl<'a> Visit<'a> for EventPublishVisitor<'a> {
    fn visit_stmt(&mut self, i: &'a Stmt) {
        if let Stmt::Local(local) = i {
            if let Some(init) = &local.init {
                if contains_storage_get(&init.expr) {
                    if let Some(name) = pat_ident_name(&local.pat) {
                        self.storage_vars.insert(name);
                    }
                }
            }
        }
        visit::visit_stmt(self, i);
    }

    fn visit_expr_method_call(&mut self, i: &'a ExprMethodCall) {
        if is_events_publish(i) {
            if let Some(data_arg) = i.args.iter().nth(1) {
                if expr_carries_storage_value(data_arg, &self.storage_vars) {
                    self.out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::Low,
                        file_path: String::new(),
                        line: i.span().start().line,
                        function_name: self.fn_name.clone(),
                        description: format!(
                            "Event data in `{}` contains a full storage value from `get()`. \
                             Publish only meaningful deltas to reduce data leakage and storage costs.",
                            self.fn_name
                        ),
                    });
                }
            }
        }
        visit::visit_expr_method_call(self, i);
    }
}

fn pat_ident_name(pat: &syn::Pat) -> Option<String> {
    match pat {
        syn::Pat::Ident(pat_ident) => Some(pat_ident.ident.to_string()),
        syn::Pat::Type(pat_type) => pat_ident_name(&pat_type.pat),
        _ => None,
    }
}

fn is_events_publish(m: &ExprMethodCall) -> bool {
    if m.method != "publish" {
        return false;
    }
    receiver_chain_contains_events(&m.receiver)
}

fn receiver_chain_contains_events(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            if m.method == "events" {
                return true;
            }
            receiver_chain_contains_events(&m.receiver)
        }
        Expr::Field(f) => receiver_chain_contains_events(&f.base),
        _ => false,
    }
}

/// Whether an expression, possibly wrapped in `.unwrap()`/`.unwrap_or(...)`, directly
/// contains a storage `get()` call somewhere in its receiver chain.
fn contains_storage_get(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            if m.method == "get" && receiver_chain_contains_storage(&m.receiver) {
                return true;
            }
            contains_storage_get(&m.receiver)
        }
        _ => false,
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

/// Whether an event-data expression carries a storage value: either a direct storage
/// `get()` chain, a tracked variable assigned from one, or a tuple containing either.
fn expr_carries_storage_value(expr: &Expr, storage_vars: &HashSet<String>) -> bool {
    match expr {
        Expr::Tuple(t) => t
            .elems
            .iter()
            .any(|e| expr_carries_storage_value(e, storage_vars)),
        Expr::Reference(r) => expr_carries_storage_value(&r.expr, storage_vars),
        Expr::Path(p) => p
            .path
            .get_ident()
            .map(|id| storage_vars.contains(&id.to_string()))
            .unwrap_or(false),
        _ => contains_storage_get(expr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_file;

    #[test]
    fn flags_event_with_full_storage_value() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn process(env: Env) {
        let val = env.storage().persistent().get(&"key").unwrap_or(0);
        env.events().publish(("state",), (val,));
    }
}
"#,
        )?;
        let hits = EventFullStateCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::Low);
        Ok(())
    }

    #[test]
    fn passes_event_with_computed_delta() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn process(env: Env) {
        let old_val = env.storage().persistent().get(&"key").unwrap_or(0);
        let new_val = old_val + 10;
        env.storage().persistent().set(&"key", &new_val);
        env.events().publish(("delta",), (10,));
    }
}
"#,
        )?;
        let hits = EventFullStateCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn passes_event_with_literal_data() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn process(env: Env) {
        env.events().publish(("event",), (42,));
    }
}
"#,
        )?;
        let hits = EventFullStateCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }
}
