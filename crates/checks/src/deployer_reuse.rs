//! Detects `env.deployer()` result cached in a variable and reused across multiple `deploy()` calls.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File, Local, Pat, Stmt};

const CHECK_NAME: &str = "deployer-reuse";

pub struct DeployerReuseCheck;

impl Check for DeployerReuseCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut candidates: Vec<Finding> = Vec::new();

        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut scan = DeployerReuseScan {
                fn_name,
                deployer_vars: Vec::new(),
                out: &mut candidates,
            };
            scan.visit_block(&method.block);
        }

        // Only keep findings from functions where a cached deployer is used for 2+ deploy() calls.
        let mut counts: HashMap<String, usize> = HashMap::new();
        for f in &candidates {
            *counts.entry(f.function_name.clone()).or_insert(0) += 1;
        }
        candidates
            .into_iter()
            .filter(|f| counts.get(&f.function_name).copied().unwrap_or(0) > 1)
            .collect()
    }
}

/// Returns `true` if `expr` is `env.deployer()`.
fn is_env_deployer_call(expr: &Expr) -> bool {
    if let Expr::MethodCall(m) = expr {
        if m.method == "deployer" {
            if let Expr::Path(p) = &*m.receiver {
                return p.path.is_ident("env");
            }
        }
    }
    false
}

/// If `local` is `let <ident> = env.deployer()`, return the ident name.
fn deployer_binding(local: &Local) -> Option<String> {
    let init = local.init.as_ref()?;
    if !is_env_deployer_call(&init.expr) {
        return None;
    }
    if let Pat::Ident(pat_ident) = &local.pat {
        return Some(pat_ident.ident.to_string());
    }
    None
}

struct DeployerReuseScan<'a> {
    fn_name: String,
    /// Variable names bound to `env.deployer()`.
    deployer_vars: Vec<String>,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for DeployerReuseScan<'_> {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        if let Stmt::Local(local) = stmt {
            if let Some(var_name) = deployer_binding(local) {
                self.deployer_vars.push(var_name);
            }
        }
        visit::visit_stmt(self, stmt);
    }

    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if i.method == "deploy" {
            if let Expr::Path(p) = &*i.receiver {
                if let Some(seg) = p.path.segments.last() {
                    let var_name = seg.ident.to_string();
                    if self.deployer_vars.contains(&var_name) {
                        self.out.push(Finding {
                            check_name: CHECK_NAME.to_string(),
                            severity: Severity::Low,
                            file_path: String::new(),
                            line: i.span().start().line,
                            function_name: self.fn_name.clone(),
                            description: format!(
                                "Function `{}` calls `.deploy()` on a cached `env.deployer()` result (`{}`). \
                                 Reusing a stored deployer reference across multiple deploy calls may produce \
                                 incorrect results if the deployer state becomes stale.",
                                self.fn_name, var_name
                            ),
                        });
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
    use syn::parse_file;

    fn run(code: &str) -> Vec<Finding> {
        let file = parse_file(code).unwrap();
        DeployerReuseCheck.run(&file, code)
    }

    #[test]
    fn flags_cached_deployer_used_twice() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn deploy_two(env: Env, wasm_a: BytesN<32>, wasm_b: BytesN<32>) {
        let deployer = env.deployer();
        deployer.deploy(wasm_a, &[]);
        deployer.deploy(wasm_b, &[]);
    }
}
"#;
        let findings = run(code);
        assert_eq!(findings.len(), 2, "expected 2 findings (one per deploy call)");
        assert!(findings.iter().all(|f| f.check_name == CHECK_NAME));
        assert!(findings.iter().all(|f| f.severity == Severity::Low));
    }

    #[test]
    fn no_flag_for_single_deploy_on_cached_deployer() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn deploy_one(env: Env, wasm: BytesN<32>) {
        let deployer = env.deployer();
        deployer.deploy(wasm, &[]);
    }
}
"#;
        let findings = run(code);
        assert!(findings.is_empty(), "single deploy on cached deployer is fine");
    }

    #[test]
    fn no_flag_for_inline_deployer_calls() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn deploy_inline(env: Env, wasm_a: BytesN<32>, wasm_b: BytesN<32>) {
        env.deployer().deploy(wasm_a, &[]);
        env.deployer().deploy(wasm_b, &[]);
    }
}
"#;
        let findings = run(code);
        assert!(
            findings.is_empty(),
            "inline deployer() calls each create a fresh deployer — not flagged"
        );
    }

    #[test]
    fn flags_three_deploys_on_cached_deployer() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn deploy_three(env: Env, a: BytesN<32>, b: BytesN<32>, c: BytesN<32>) {
        let d = env.deployer();
        d.deploy(a, &[]);
        d.deploy(b, &[]);
        d.deploy(c, &[]);
    }
}
"#;
        let findings = run(code);
        assert_eq!(findings.len(), 3);
    }
}
