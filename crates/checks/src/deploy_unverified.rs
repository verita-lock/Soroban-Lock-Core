//! Flags `env.deployer().deploy()` return values that are used without `env.is_contract()` verification.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, Pat, Stmt};

const CHECK_NAME: &str = "deploy-unverified";

pub struct DeployUnverifiedCheck;

impl Check for DeployUnverifiedCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut visitor = DeployUnverifiedVisitor {
                fn_name,
                deploy_vars: Vec::new(),
                verified: HashSet::new(),
            };
            visitor.visit_block(&method.block);
            for (var_name, line) in visitor.deploy_vars {
                if !visitor.verified.contains(&var_name) {
                    out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::Low,
                        file_path: String::new(),
                        line,
                        function_name: visitor.fn_name.clone(),
                        description: format!(
                            "Method `{}` deploys a sub-contract and uses the returned address without verifying that it exists via `env.is_contract(...)`.",
                            visitor.fn_name
                        ),
                    });
                }
            }
        }
        out
    }
}

fn is_deploy_call(m: &ExprMethodCall) -> bool {
    if m.method != "deploy" {
        return false;
    }
    matches!(&*m.receiver, Expr::MethodCall(inner) if inner.method == "deployer")
}

fn is_is_contract_call(m: &ExprMethodCall) -> bool {
    if m.method != "is_contract" {
        return false;
    }
    matches!(&*m.receiver, Expr::Path(p) if p.path.is_ident("env"))
}

fn expr_ident_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(path) => path.path.get_ident().map(|id| id.to_string()),
        Expr::Reference(r) => expr_ident_name(&r.expr),
        Expr::Paren(p) => expr_ident_name(&p.expr),
        _ => None,
    }
}

struct DeployUnverifiedVisitor {
    fn_name: String,
    deploy_vars: Vec<(String, usize)>,
    verified: HashSet<String>,
}

impl<'ast> Visit<'ast> for DeployUnverifiedVisitor {
    fn visit_stmt(&mut self, i: &'ast Stmt) {
        if let Stmt::Local(local) = i {
            if let Some(init_expr) = &local.init {
                if let Expr::MethodCall(m) = &*init_expr.expr {
                    if is_deploy_call(m) {
                        if let Pat::Ident(pi) = &local.pat {
                            self.deploy_vars.push((pi.ident.to_string(), m.span().start().line));
                        }
                    }
                }
            }
        }
        visit::visit_stmt(self, i);
    }

    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_is_contract_call(i) {
            if let Some(arg) = i.args.first() {
                if let Some(name) = expr_ident_name(arg) {
                    self.verified.insert(name);
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
        DeployUnverifiedCheck.run(&parse_file(src).unwrap(), src)
    }

    #[test]
    fn flags_deploy_without_is_contract() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Bytes, Env, Symbol};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn deploy(env: Env, wasm_hash: Bytes) {
        let addr = env.deployer().deploy(wasm_hash, ());
        env.storage().persistent().set(&Symbol::new(&env, "addr"), &addr);
    }
}
"#);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].check_name, CHECK_NAME);
    }

    #[test]
    fn passes_when_is_contract_called() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Bytes, Env, Symbol};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn deploy(env: Env, wasm_hash: Bytes) {
        let addr = env.deployer().deploy(wasm_hash, ());
        if env.is_contract(&addr) {
            env.storage().persistent().set(&Symbol::new(&env, "addr"), &addr);
        }
    }
}
"#);
        assert!(hits.is_empty());
    }
}
