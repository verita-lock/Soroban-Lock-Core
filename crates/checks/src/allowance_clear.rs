//! Detects `transfer_from` calls not followed by an `approve` to decrement the allowance.
//!
//! After `token.transfer_from(spender, from, to, amount)` the spender's allowance
//! must be reduced (via a subsequent `token.approve(..., remaining, ...)` or
//! `token.approve(..., 0, ...)`).  Omitting this step lets the spender reuse the
//! same allowance indefinitely, draining the `from` account.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Block, ExprMethodCall, File};

const CHECK_NAME: &str = "allowance-not-cleared";

pub struct AllowanceClearCheck;

impl Check for AllowanceClearCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            for line in flagged_lines(&method.block) {
                out.push(Finding {
                    check_name: CHECK_NAME.to_string(),
                    severity: Severity::High,
                    file_path: String::new(),
                    line,
                    function_name: fn_name.clone(),
                    description: "Call to `transfer_from` is not followed by `approve` to \
                                  decrement the spender's allowance. The spender can reuse \
                                  the same allowance indefinitely, draining the account."
                        .to_string(),
                });
            }
        }
        out
    }
}

/// Returns source lines of `transfer_from` calls in `block` that have no
/// accompanying `approve` call anywhere in the same block.
fn flagged_lines(block: &Block) -> Vec<usize> {
    let mut scan = Scan::default();
    scan.visit_block(block);
    if scan.has_approve {
        vec![]
    } else {
        scan.transfer_from_lines
    }
}

#[derive(Default)]
struct Scan {
    transfer_from_lines: Vec<usize>,
    has_approve: bool,
}

impl<'ast> Visit<'ast> for Scan {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        match i.method.to_string().as_str() {
            "transfer_from" => {
                self.transfer_from_lines.push(i.method.span().start().line);
            }
            "approve" => {
                self.has_approve = true;
            }
            _ => {}
        }
        visit::visit_expr_method_call(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_file;

    fn run(src: &str) -> Vec<Finding> {
        AllowanceClearCheck.run(&parse_file(src).unwrap(), src)
    }

    #[test]
    fn flags_transfer_from_without_approve() {
        let hits = run(r#"
pub struct C;
#[contractimpl]
impl C {
    pub fn spend(env: Env, token: Address, spender: Address, from: Address, to: Address, amount: i128) {
        let client = token::Client::new(&env, &token);
        client.transfer_from(&spender, &from, &to, &amount);
    }
}
"#);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::High);
        assert_eq!(hits[0].check_name, CHECK_NAME);
        assert_eq!(hits[0].function_name, "spend");
    }

    #[test]
    fn passes_when_approve_follows_transfer_from() {
        let hits = run(r#"
pub struct C;
#[contractimpl]
impl C {
    pub fn spend(env: Env, token: Address, spender: Address, from: Address, to: Address, amount: i128) {
        let client = token::Client::new(&env, &token);
        client.transfer_from(&spender, &from, &to, &amount);
        client.approve(&from, &spender, &0_i128, &0_u32);
    }
}
"#);
        assert!(hits.is_empty());
    }

    #[test]
    fn passes_when_no_transfer_from() {
        let hits = run(r#"
pub struct C;
#[contractimpl]
impl C {
    pub fn transfer(env: Env, token: Address, from: Address, to: Address, amount: i128) {
        let client = token::Client::new(&env, &token);
        client.transfer(&from, &to, &amount);
    }
}
"#);
        assert!(hits.is_empty());
    }

    #[test]
    fn ignores_non_contractimpl() {
        let hits = run(r#"
pub struct C;
impl C {
    pub fn spend(env: Env, token: Address, spender: Address, from: Address, to: Address, amount: i128) {
        let client = token::Client::new(&env, &token);
        client.transfer_from(&spender, &from, &to, &amount);
    }
}
"#);
        assert!(hits.is_empty());
    }

    #[test]
    fn flags_multiple_transfer_from_without_approve() {
        let hits = run(r#"
pub struct C;
#[contractimpl]
impl C {
    pub fn batch(env: Env, token: Address, spender: Address, from: Address, to: Address, amount: i128) {
        let client = token::Client::new(&env, &token);
        client.transfer_from(&spender, &from, &to, &amount);
        client.transfer_from(&spender, &from, &to, &amount);
    }
}
"#);
        assert_eq!(hits.len(), 2);
    }
}
