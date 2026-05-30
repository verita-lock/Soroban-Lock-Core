//! Detects untrusted casts or conversions of `env.invoke_contract()` return values.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprIf, ExprMatch, ExprMethodCall, File, Stmt};

const CHECK_NAME: &str = "invoke-result-untrusted";

pub struct InvokeResultUntrustedCheck;

impl Check for InvokeResultUntrustedCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut visitor = InvokeResultUntrustedVisitor {
                fn_name,
                invoke_bindings: Vec::new(),
                out: &mut out,
                safe_context: false,
            };
            visitor.visit_block(&method.block);
        }
        out
    }
}

fn is_invoke_contract_call(m: &ExprMethodCall) -> bool {
    m.method == "invoke_contract"
        && matches!(&*m.receiver, Expr::Path(p) if p.path.is_ident("env"))
}

fn is_cast_method(m: &ExprMethodCall) -> bool {
    matches!(
        m.method.to_string().as_str(),
        "try_into_val" | "from_val" | "try_from_val"
    )
}

fn expr_contains_invoke_contract(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            if is_invoke_contract_call(m) {
                return true;
            }
            expr_contains_invoke_contract(&m.receiver)
                || m.args.iter().any(expr_contains_invoke_contract)
        }
        Expr::Reference(r) => expr_contains_invoke_contract(&r.expr),
        Expr::Paren(p) => expr_contains_invoke_contract(&p.expr),
        _ => false,
    }
}

fn expr_ident_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(path) => path.path.get_ident().map(|id| id.to_string()),
        Expr::Reference(r) => expr_ident_name(&r.expr),
        Expr::Paren(p) => expr_ident_name(&p.expr),
        _ => None,
    }
}

struct InvokeResultUntrustedVisitor<'a> {
    fn_name: String,
    invoke_bindings: Vec<String>,
    out: &'a mut Vec<Finding>,
    safe_context: bool,
}

impl<'ast> Visit<'ast> for InvokeResultUntrustedVisitor<'_> {
    fn visit_stmt(&mut self, i: &'ast Stmt) {
        if let Stmt::Local(local) = i {
            if let Some(init_expr) = &local.init {
                let mut expr = &init_expr.expr;
                if let Expr::MethodCall(cast_call) = &**expr {
                    if is_cast_method(cast_call)
                        && expr_contains_invoke_contract(&cast_call.receiver)
                    {
                        if let syn::Pat::Ident(pi) = &local.pat {
                            self.invoke_bindings.push(pi.ident.to_string());
                        }
                    }
                }
            }
        }
        visit::visit_stmt(self, i);
    }

    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_cast_method(i) && !self.safe_context {
            let receiver_contains_invoke = expr_contains_invoke_contract(&i.receiver);
            let receiver_is_invoke_binding = expr_ident_name(&i.receiver)
                .map_or(false, |name| self.invoke_bindings.contains(&name));
            if receiver_contains_invoke || receiver_is_invoke_binding {
                self.out.push(Finding {
                    check_name: CHECK_NAME.to_string(),
                    severity: Severity::Medium,
                    file_path: String::new(),
                    line: i.span().start().line,
                    function_name: self.fn_name.clone(),
                    description: format!(
                        "Method `{}` converts the result of `invoke_contract` with `{}` without validating the returned schema. Validate the result before converting or use a `match`/`if let` guard.",
                        self.fn_name,
                        i.method
                    ),
                });
            }
        }
        visit::visit_expr_method_call(self, i);
    }

    fn visit_expr_match(&mut self, i: &'ast ExprMatch) {
        let prev = self.safe_context;
        self.safe_context = true;
        self.visit_expr(&i.expr);
        self.safe_context = prev;
        for arm in &i.arms {
            self.visit_pat(&arm.pat);
            if let Some((_, guard)) = &arm.guard {
                self.visit_expr(guard);
            }
            self.visit_expr(&arm.body);
        }
    }

    fn visit_expr_if(&mut self, i: &'ast ExprIf) {
        if let Expr::Let(expr_let) = &*i.cond {
            let prev = self.safe_context;
            self.safe_context = true;
            self.visit_expr(&expr_let.expr);
            self.safe_context = prev;
            self.visit_pat(&expr_let.pat);
            self.visit_block(&i.then_branch);
            if let Some((_, else_branch)) = &i.else_branch {
                self.visit_expr(else_branch);
            }
        } else {
            visit::visit_expr_if(self, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    fn run(src: &str) -> Vec<Finding> {
        InvokeResultUntrustedCheck.run(&parse_file(src).unwrap(), src)
    }

    #[test]
    fn flags_inline_try_into_val() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn call(env: Env, contract: Address) -> i128 {
        env.invoke_contract::<Val>(&contract, &Symbol::short("get"), &())
            .try_into_val(&env)
    }
}
"#);
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn flags_bound_try_into_val() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn call(env: Env, contract: Address) -> i128 {
        let result = env.invoke_contract::<Val>(&contract, &Symbol::short("get"), &());
        result.try_into_val(&env)
    }
}
"#);
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn passes_when_match_used() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn call(env: Env, contract: Address) -> Option<i128> {
        let result: Val = env.invoke_contract(&contract, &Symbol::short("get"), &());
        match result.try_into_val(&env) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
"#);
        assert!(hits.is_empty());
    }
}
