//! Admin address compared against caller instead of using require_auth.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprBinary, ExprMethodCall, File, FnArg, Pat, Type};

const CHECK_NAME: &str = "address-cmp-instead-of-auth";

/// Detects patterns where an Address parameter is compared with == against a stored address
/// instead of calling require_auth on the stored address.
pub struct AddressCmpInsteadOfAuthCheck;

impl Check for AddressCmpInsteadOfAuthCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let addr_params = extract_address_params(&method.sig.inputs);
            if addr_params.is_empty() {
                continue;
            }
            let mut scan = AddressCmpScan {
                fn_name: fn_name.clone(),
                addr_params: addr_params.clone(),
                out: &mut out,
                has_require_auth: false,
            };
            scan.visit_block(&method.block);
        }
        out
    }
}

fn extract_address_params(inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>) -> Vec<String> {
    let mut params = Vec::new();
    for arg in inputs {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                if is_address_type(&pat_type.ty) {
                    params.push(pat_ident.ident.to_string());
                }
            }
        }
    }
    params
}

fn is_address_type(ty: &Type) -> bool {
    match ty {
        Type::Path(p) => {
            if let Some(seg) = p.path.segments.last() {
                seg.ident == "Address"
            } else {
                false
            }
        }
        _ => false,
    }
}

fn is_require_auth_call(m: &ExprMethodCall) -> bool {
    (m.method == "require_auth" || m.method == "require_auth_for_args")
        && matches!(&*m.receiver, Expr::Path(p) if p.path.is_ident("env"))
}

fn contains_address_comparison(expr: &Expr, addr_params: &[String]) -> bool {
    match expr {
        Expr::Binary(bin) => {
            matches!(bin.op, syn::BinOp::Eq(_) | syn::BinOp::Ne(_))
                && (is_address_param(&bin.left, addr_params) || is_address_param(&bin.right, addr_params))
        }
        _ => false,
    }
}

fn is_address_param(expr: &Expr, addr_params: &[String]) -> bool {
    match expr {
        Expr::Path(p) => {
            if let Some(seg) = p.path.segments.last() {
                addr_params.contains(&seg.ident.to_string())
            } else {
                false
            }
        }
        _ => false,
    }
}

struct AddressCmpScan<'a> {
    fn_name: String,
    addr_params: Vec<String>,
    out: &'a mut Vec<Finding>,
    has_require_auth: bool,
}

impl<'ast> Visit<'ast> for AddressCmpScan<'_> {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_require_auth_call(i) {
            self.has_require_auth = true;
        }
        visit::visit_expr_method_call(self, i);
    }

    fn visit_expr_binary(&mut self, i: &'ast ExprBinary) {
        if !self.has_require_auth && contains_address_comparison(&Expr::Binary(i.clone()), &self.addr_params) {
            self.out.push(Finding {
                check_name: CHECK_NAME.to_string(),
                severity: Severity::High,
                file_path: String::new(),
                line: i.span().start().line,
                function_name: self.fn_name.clone(),
                description: format!(
                    "Function `{}` compares an Address parameter with == instead of calling require_auth(). \
                     This bypasses Soroban's host-level signature verification. Use require_auth() on the stored address instead.",
                    self.fn_name
                ),
            });
        }
        visit::visit_expr_binary(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_file;

    #[test]
    fn detects_address_comparison_without_auth() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn vulnerable(env: Env, caller: Address) {
        let admin = env.storage().instance().get(&Symbol::new(&env, "admin")).unwrap();
        if caller == admin {
            // do something
        }
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = AddressCmpInsteadOfAuthCheck;
        let findings = check.run(&file, code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].check_name, CHECK_NAME);
        assert_eq!(findings[0].severity, Severity::High);
    }

    #[test]
    fn allows_require_auth() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn safe(env: Env, caller: Address) {
        env.require_auth(&caller);
        let admin = env.storage().instance().get(&Symbol::new(&env, "admin")).unwrap();
        if caller == admin {
            // do something
        }
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = AddressCmpInsteadOfAuthCheck;
        let findings = check.run(&file, code);
        assert!(findings.is_empty());
    }

    #[test]
    fn ignores_non_address_comparison() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn safe(env: Env) {
        if 1 == 2 {
            // do something
        }
    }
}
        "#;
        let file = parse_file(code).unwrap();
        let check = AddressCmpInsteadOfAuthCheck;
        let findings = check.run(&file, code);
        assert!(findings.is_empty());
    }
}
