//! Division (`/`) or remainder (`%`) by a user-supplied parameter without a zero guard.
//! An attacker can pass `0` as the divisor to panic the entire transaction.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use quote::ToTokens;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{BinOp, Expr, ExprBinary, File, FnArg, Macro, Pat};

const CHECK_NAME: &str = "zero-divisor";

/// Flags `#[contractimpl]` methods where a parameter is used as the divisor in `/` or `%`
/// without an `assert!(param != 0, ...)` or `if param ... 0` guard anywhere in the body.
pub struct ZeroDivisorCheck;

impl Check for ZeroDivisorCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let param_idents: HashSet<String> = method
                .sig
                .inputs
                .iter()
                .filter_map(|arg| {
                    if let FnArg::Typed(pat_type) = arg {
                        if let Pat::Ident(pat_ident) = &*pat_type.pat {
                            return Some(pat_ident.ident.to_string());
                        }
                    }
                    None
                })
                .collect();

            if param_idents.is_empty() {
                continue;
            }

            // Determine which params have guards (assert! or if-condition mentioning param + "0")
            let mut guard_collector = GuardCollector {
                param_idents: &param_idents,
                guarded: HashSet::new(),
            };
            guard_collector.visit_block(&method.block);
            let guarded = guard_collector.guarded;

            // Visit binary expressions and flag unguarded divisor params
            let mut reported = HashSet::new();
            let mut div_visitor = DivisorVisitor {
                fn_name: &fn_name,
                param_idents: &param_idents,
                guarded: &guarded,
                reported: &mut reported,
                out: &mut out,
            };
            div_visitor.visit_block(&method.block);
        }
        out
    }
}

/// Collects which parameter idents have a guard in the function body.
struct GuardCollector<'a> {
    param_idents: &'a HashSet<String>,
    guarded: HashSet<String>,
}

impl<'ast> Visit<'ast> for GuardCollector<'_> {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        if mac.path.is_ident("assert") {
            let tokens_str = mac.tokens.to_string();
            for param in self.param_idents.iter() {
                if tokens_str.contains(param.as_str()) {
                    self.guarded.insert(param.clone());
                }
            }
        }
        visit::visit_macro(self, mac);
    }

    fn visit_expr(&mut self, i: &'ast Expr) {
        if let Expr::Macro(m) = i {
            if m.mac.path.is_ident("assert") {
                let tokens_str = m.mac.tokens.to_string();
                for param in self.param_idents.iter() {
                    if tokens_str.contains(param.as_str()) {
                        self.guarded.insert(param.clone());
                    }
                }
            }
        }
        if let Expr::If(if_expr) = i {
            let cond_str = if_expr.cond.to_token_stream().to_string();
            for param in self.param_idents.iter() {
                if cond_str.contains(param.as_str()) && cond_str.contains("0") {
                    self.guarded.insert(param.clone());
                }
            }
        }
        visit::visit_expr(self, i);
    }
}

/// Visits binary expressions to find Div/Rem by an unguarded parameter.
struct DivisorVisitor<'a> {
    fn_name: &'a str,
    param_idents: &'a HashSet<String>,
    guarded: &'a HashSet<String>,
    reported: &'a mut HashSet<String>,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for DivisorVisitor<'_> {
    fn visit_expr_binary(&mut self, i: &'ast ExprBinary) {
        match &i.op {
            BinOp::Div(_) | BinOp::Rem(_) => {
                if let Expr::Path(path) = &*i.right {
                    if let Some(ident) = path.path.get_ident() {
                        let param_name = ident.to_string();
                        if self.param_idents.contains(&param_name)
                            && !self.guarded.contains(&param_name)
                            && self.reported.insert(param_name.clone())
                        {
                            let op_str = match &i.op {
                                BinOp::Div(_) => "/",
                                _ => "%",
                            };
                            self.out.push(Finding {
                                check_name: CHECK_NAME.to_string(),
                                severity: Severity::High,
                                file_path: String::new(),
                                line: i.span().start().line,
                                function_name: self.fn_name.to_string(),
                                description: format!(
                                    "Parameter `{param_name}` is used as the divisor (`{op_str}`) in \
                                     `{fn_name}` without a zero-check guard. An attacker who passes \
                                     0 will panic the entire transaction.",
                                    param_name = param_name,
                                    fn_name = self.fn_name,
                                    op_str = op_str,
                                ),
                            });
                        }
                    }
                }
            }
            _ => {}
        }
        visit::visit_expr_binary(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_file;

    #[test]
    fn flags_div_by_param_without_guard() {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn quote(env: Env, amount: i128, rate: i128) -> i128 {
        let _ = env;
        amount / rate
    }
}
"#,
        )
        .unwrap();
        let hits = ZeroDivisorCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].check_name, CHECK_NAME);
        assert_eq!(hits[0].severity, Severity::High);
        assert_eq!(hits[0].function_name, "quote");
    }

    #[test]
    fn passes_with_assert_guard() {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn quote(env: Env, amount: i128, rate: i128) -> i128 {
        let _ = env;
        assert!(rate != 0, "rate must be nonzero");
        amount / rate
    }
}
"#,
        )
        .unwrap();
        let hits = ZeroDivisorCheck.run(&file, "");
        assert!(hits.is_empty());
    }

    #[test]
    fn passes_with_if_guard() {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn quote(env: Env, amount: i128, rate: i128) -> i128 {
        let _ = env;
        if rate == 0 {
            panic!("rate must be nonzero");
        }
        amount / rate
    }
}
"#,
        )
        .unwrap();
        let hits = ZeroDivisorCheck.run(&file, "");
        assert!(hits.is_empty());
    }

    #[test]
    fn flags_remainder_without_guard() {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn distribute(env: Env, total: i128, divisor: i128) -> i128 {
        let _ = env;
        total % divisor
    }
}
"#,
        )
        .unwrap();
        let hits = ZeroDivisorCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].function_name, "distribute");
    }

    #[test]
    fn flags_only_first_per_unguarded_param() {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn multi(env: Env, a: i128, b: i128) -> i128 {
        let _ = env;
        let _ = a / b;
        let _ = a / b;
    }
}
"#,
        )
        .unwrap();
        let hits = ZeroDivisorCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn flags_both_params_when_both_unguarded() {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn multi_div(env: Env, x: i128, y: i128, z: i128) -> i128 {
        let _ = env;
        let _ = x / y;
        let _ = x / z;
    }
}
"#,
        )
        .unwrap();
        let hits = ZeroDivisorCheck.run(&file, "");
        assert_eq!(hits.len(), 2);
    }

    #[test]
    fn ignores_non_contractimpl() {
        let file = parse_file(
            r#"
use soroban_sdk::Env;

pub struct C;

impl C {
    pub fn quote(env: Env, amount: i128, rate: i128) -> i128 {
        let _ = env;
        amount / rate
    }
}
"#,
        )
        .unwrap();
        let hits = ZeroDivisorCheck.run(&file, "");
        assert!(hits.is_empty());
    }
}
