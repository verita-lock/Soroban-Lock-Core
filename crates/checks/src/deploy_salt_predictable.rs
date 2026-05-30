//! Flags `env.deployer().deploy()` calls with predictable salt derived from ledger data.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, Pat, Stmt};

const CHECK_NAME: &str = "deploy-salt-predictable";

pub struct DeploySaltPredictableCheck;

impl Check for DeploySaltPredictableCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut visitor = DeploySaltPredictableVisitor {
                fn_name,
                ledger_vars: Vec::new(),
                out: &mut out,
            };
            visitor.visit_block(&method.block);
        }
        out
    }
}

fn is_ledger_rand(expr: &Expr) -> bool {
    let Expr::MethodCall(outer) = expr else {
        return false;
    };
    if !matches!(outer.method.to_string().as_str(), "timestamp" | "sequence") {
        return false;
    }
    let Expr::MethodCall(inner) = &*outer.receiver else {
        return false;
    };
    inner.method == "ledger"
}

fn expr_contains_ledger_rand(expr: &Expr, ledger_vars: &[String]) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            if is_ledger_rand(expr) {
                return true;
            }
            expr_contains_ledger_rand(&m.receiver, ledger_vars)
                || m.args
                    .iter()
                    .any(|arg| expr_contains_ledger_rand(arg, ledger_vars))
        }
        Expr::Path(path) => path
            .path
            .get_ident()
            .map_or(false, |id| ledger_vars.contains(&id.to_string())),
        Expr::Reference(r) => expr_contains_ledger_rand(&r.expr, ledger_vars),
        Expr::Paren(p) => expr_contains_ledger_rand(&p.expr, ledger_vars),
        _ => false,
    }
}

fn is_deploy_call(m: &ExprMethodCall) -> bool {
    if m.method != "deploy" {
        return false;
    }
    matches!(&*m.receiver, Expr::MethodCall(inner) if inner.method == "deployer")
}

struct DeploySaltPredictableVisitor<'a> {
    fn_name: String,
    ledger_vars: Vec<String>,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for DeploySaltPredictableVisitor<'_> {
    fn visit_stmt(&mut self, i: &'ast Stmt) {
        if let Stmt::Local(local) = i {
            if let Some(init_expr) = &local.init {
                if expr_contains_ledger_rand(&init_expr.expr, &self.ledger_vars) {
                    if let Pat::Ident(pi) = &local.pat {
                        self.ledger_vars.push(pi.ident.to_string());
                    }
                }
            }
        }
        visit::visit_stmt(self, i);
    }

    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_deploy_call(i) {
            if let Some(salt_arg) = i.args.iter().nth(1) {
                if expr_contains_ledger_rand(salt_arg, &self.ledger_vars) {
                    self.out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::High,
                        file_path: String::new(),
                        line: i.span().start().line,
                        function_name: self.fn_name.clone(),
                        description: format!(
                            "Method `{}` deploys a contract with a salt derived from `env.ledger().sequence()` or `env.ledger().timestamp()`. This allows deployment front-running.",
                            self.fn_name
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

    fn run(src: &str) -> Vec<Finding> {
        DeploySaltPredictableCheck.run(&parse_file(src).unwrap(), src)
    }

    #[test]
    fn flags_ledger_sequence_salt_direct() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn deploy(env: Env, wasm_hash: Bytes) {
        env.deployer().deploy(wasm_hash, env.ledger().sequence());
    }
}
"#);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::High);
    }

    #[test]
    fn flags_ledger_timestamp_salt_via_var() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn deploy(env: Env, wasm_hash: Bytes) {
        let salt = env.ledger().timestamp();
        env.deployer().deploy(wasm_hash, salt);
    }
}
"#);
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn passes_constant_salt() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn deploy(env: Env, wasm_hash: Bytes) {
        let salt = ();
        env.deployer().deploy(wasm_hash, salt);
    }
}
"#);
        assert!(hits.is_empty());
    }
}
