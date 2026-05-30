//! Burn functions that do not emit an event for the supply reduction.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File};

const CHECK_NAME: &str = "burn-no-event";

pub struct BurnNoEventCheck;

impl Check for BurnNoEventCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            if method.sig.ident != "burn" || !matches!(method.vis, syn::Visibility::Public(_)) {
                continue;
            }

            let mut scan = EventScan::default();
            scan.visit_block(&method.block);

            if !scan.events_publish {
                out.push(Finding {
                    check_name: CHECK_NAME.to_string(),
                    severity: Severity::Low,
                    file_path: String::new(),
                    line: method.sig.ident.span().start().line,
                    function_name: "burn".to_string(),
                    description: "Method `burn` decreases token supply but does not emit an event via `env.events().publish(...)`. Off-chain indexers cannot account for the burn without an event.".to_string(),
                });
            }
        }
        out
    }
}

fn is_events_publish(m: &ExprMethodCall) -> bool {
    m.method == "publish" && receiver_chain_contains_events(&m.receiver)
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

#[derive(Default)]
struct EventScan {
    events_publish: bool,
}

impl<'ast> Visit<'ast> for EventScan {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_events_publish(i) {
            self.events_publish = true;
        }
        visit::visit_expr_method_call(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    fn run(src: &str) -> Vec<Finding> {
        BurnNoEventCheck.run(&parse_file(src).unwrap(), src)
    }

    #[test]
    fn flags_burn_without_event() {
        let hits = run(r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn burn(env: Env, from: Address, amount: i128) {
        env.storage().persistent().set(&symbol_short!("supply"), &(0));
    }
}
"#);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].function_name, "burn");
        assert_eq!(hits[0].severity, Severity::Low);
    }

    #[test]
    fn passes_when_burn_emits_event() {
        let hits = run(r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn burn(env: Env, from: Address, amount: i128) {
        env.events().publish((symbol_short!("burn"),), amount);
        env.storage().persistent().set(&symbol_short!("supply"), &(0));
    }
}
"#);
        assert!(hits.is_empty());
    }
}
