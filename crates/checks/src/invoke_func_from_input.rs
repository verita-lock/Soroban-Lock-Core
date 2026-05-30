//! Detects `invoke_contract` calls where the function name symbol is derived from user input.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMacro, ExprMethodCall, File, FnArg, Pat, PatType, Stmt};

const CHECK_NAME: &str = "invoke-func-from-input";

pub struct InvokeFuncFromInputCheck;

impl Check for InvokeFuncFromInputCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let params = parameter_names(&method.sig.inputs);
            if params.is_empty() {
                continue;
            }
            let mut v = InvokeFuncFromInputVisitor {
                fn_name: method.sig.ident.to_string(),
                params: &params,
                symbol_vars: Vec::new(),
                out: &mut out,
            };
            v.visit_block(&method.block);
        }
        out
    }
}

fn parameter_names(inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>) -> HashSet<String> {
    inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(PatType { pat, .. }) = arg {
                if let Pat::Ident(pi) = pat.as_ref() {
                    let name = pi.ident.to_string();
                    if name != "env" {
                        return Some(name);
                    }
                }
            }
            None
        })
        .collect()
}

fn expr_uses_param(expr: &Expr, params: &HashSet<String>) -> bool {
    match expr {
        Expr::Path(p) => p.path.get_ident().map_or(false, |id| params.contains(&id.to_string())),
        Expr::Reference(r) => expr_uses_param(&r.expr, params),
        Expr::Paren(p) => expr_uses_param(&p.expr, params),
        Expr::MethodCall(m) => expr_uses_param(&m.receiver, params)
            || m.args.iter().any(|arg| expr_uses_param(arg, params)),
        Expr::Call(c) => expr_uses_param(&c.func, params)
            || c.args.iter().any(|arg| expr_uses_param(arg, params)),
        Expr::Field(f) => expr_uses_param(&f.base, params),
        Expr::Macro(m) => macro_contains_param(m, params),
        Expr::Tuple(t) => t.elems.iter().any(|e| expr_uses_param(e, params)),
        Expr::Array(a) => a.elems.iter().any(|e| expr_uses_param(e, params)),
        Expr::Binary(b) => {
            expr_uses_param(&b.left, params) || expr_uses_param(&b.right, params)
        }
        Expr::Unary(u) => expr_uses_param(&u.expr, params),
        Expr::Cast(c) => expr_uses_param(&c.expr, params),
        Expr::Reference(r) => expr_uses_param(&r.expr, params),
        Expr::Try(t) => expr_uses_param(&t.expr, params),
        Expr::Match(m) => expr_uses_param(&m.expr, params),
        _ => false,
    }
}

fn macro_contains_param(mac: &ExprMacro, params: &HashSet<String>) -> bool {
    let tokens = mac.mac.tokens.to_string();
    params.iter().any(|param| tokens.contains(param))
}

fn is_symbol_short_with_param(expr: &Expr, params: &HashSet<String>) -> bool {
    match expr {
        Expr::Macro(m) => m
            .mac
            .path
            .segments
            .last()
            .is_some_and(|s| s.ident == "symbol_short")
            && macro_contains_param(m, params),
        Expr::Reference(r) => is_symbol_short_with_param(&r.expr, params),
        Expr::Paren(p) => is_symbol_short_with_param(&p.expr, params),
        _ => false,
    }
}

fn is_symbol_from_str_with_param(expr: &Expr, params: &HashSet<String>) -> bool {
    let Expr::Call(call) = expr else {
        return false;
    };
    let Expr::Path(func) = &*call.func else {
        return false;
    };
    if func.path.segments.last().is_some_and(|s| s.ident == "from_str") {
        if let Some(arg) = call.args.iter().nth(1) {
            return expr_uses_param(arg, params);
        }
    }
    false
}

fn is_user_input_symbol(expr: &Expr, params: &HashSet<String>) -> bool {
    is_symbol_short_with_param(expr, params) || is_symbol_from_str_with_param(expr, params)
}

fn expr_ident_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(path) => path.path.get_ident().map(|id| id.to_string()),
        Expr::Reference(r) => expr_ident_name(&r.expr),
        Expr::Paren(p) => expr_ident_name(&p.expr),
        _ => None,
    }
}

fn is_invoke_contract_call(m: &ExprMethodCall) -> bool {
    if m.method != "invoke_contract" {
        return false;
    }
    matches!(&*m.receiver, Expr::Path(p) if p.path.is_ident("env"))
}

struct InvokeFuncFromInputVisitor<'a> {
    fn_name: String,
    params: &'a HashSet<String>,
    symbol_vars: Vec<String>,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for InvokeFuncFromInputVisitor<'_> {
    fn visit_stmt(&mut self, i: &'ast Stmt) {
        if let Stmt::Local(local) = i {
            if let Some(init_expr) = &local.init {
                if is_user_input_symbol(&init_expr.expr, self.params) {
                    if let Pat::Ident(pi) = &local.pat {
                        self.symbol_vars.push(pi.ident.to_string());
                    }
                }
            }
        }
        visit::visit_stmt(self, i);
    }

    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_invoke_contract_call(i) {
            if let Some(sym_arg) = i.args.iter().nth(1) {
                let uses_param = is_user_input_symbol(sym_arg, self.params);
                let uses_tracked_var = expr_ident_name(sym_arg)
                    .map_or(false, |name| self.symbol_vars.contains(&name));
                if uses_param || uses_tracked_var {
                    self.out.push(Finding {
                        check_name: CHECK_NAME.to_string(),
                        severity: Severity::High,
                        file_path: String::new(),
                        line: i.span().start().line,
                        function_name: self.fn_name.clone(),
                        description: format!(
                            "Method `{}` invokes a contract with a function name Symbol derived from user input. This allows callers to invoke arbitrary functions on the target contract.",
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
        InvokeFuncFromInputCheck.run(&parse_file(src).unwrap(), src)
    }

    #[test]
    fn flags_user_input_symbol_for_invoke_contract() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn call(env: Env, contract: Address, user_func: String) {
        let func_name = Symbol::from_str(&env, &user_func);
        env.invoke_contract(&contract, &func_name, &());
    }
}
"#);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].check_name, CHECK_NAME);
        assert_eq!(hits[0].severity, Severity::High);
    }

    #[test]
    fn passes_when_function_name_is_constant() {
        let hits = run(r#"
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct C;

#[contractimpl]
impl C {
    pub fn call(env: Env, contract: Address) {
        let func_name = Symbol::short("get");
        env.invoke_contract(&contract, &func_name, &());
    }
}
"#);
        assert!(hits.is_empty());
    }
}
