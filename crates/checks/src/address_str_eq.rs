//! Address compared with string equality instead of Address::eq.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{BinOp, Expr, ExprBinary, File, FnArg, Lit, Pat, Type};

const CHECK_NAME: &str = "address-str-eq";

/// Flags binary `==` expressions comparing an Address to a string literal.
pub struct AddressStrEqCheck;

impl Check for AddressStrEqCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let address_params = address_typed_params(&method.sig.inputs);
            let mut v = AddressStrEqVisitor {
                fn_name: fn_name.clone(),
                address_params,
                out: &mut out,
            };
            v.visit_block(&method.block);
        }
        out
    }
}

/// Collects names of function parameters typed `Address`.
fn address_typed_params(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
) -> HashSet<String> {
    let mut names = HashSet::new();
    for arg in inputs {
        if let FnArg::Typed(pat_type) = arg {
            if is_address_type(&pat_type.ty) {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    names.insert(pat_ident.ident.to_string());
                }
            }
        }
    }
    names
}

fn is_address_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        return tp
            .path
            .segments
            .last()
            .map(|s| s.ident == "Address")
            .unwrap_or(false);
    }
    false
}

struct AddressStrEqVisitor<'a> {
    fn_name: String,
    address_params: HashSet<String>,
    out: &'a mut Vec<Finding>,
}

impl Visit<'_> for AddressStrEqVisitor<'_> {
    fn visit_expr_binary(&mut self, i: &ExprBinary) {
        if matches!(i.op, BinOp::Eq(_)) {
            let left_is_str = is_str_literal(&i.left);
            let right_is_str = is_str_literal(&i.right);
            let left_is_addr = self.is_address_like(&i.left);
            let right_is_addr = self.is_address_like(&i.right);

            if (left_is_addr && right_is_str) || (left_is_str && right_is_addr) {
                self.out.push(Finding {
                    check_name: CHECK_NAME.to_string(),
                    severity: Severity::Low,
                    file_path: String::new(),
                    line: i.span().start().line,
                    function_name: self.fn_name.clone(),
                    description: "Expression compares an Address to a string literal with `==`. \
                         Use `Address::eq` or derive-compatible `PartialEq` instead."
                        .to_string(),
                });
            }
        }
        visit::visit_expr_binary(self, i);
    }
}

impl AddressStrEqVisitor<'_> {
    fn is_address_like(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Path(p) => p
                .path
                .get_ident()
                .map(|id| self.address_params.contains(&id.to_string()))
                .unwrap_or(false),
            Expr::Field(f) => match &f.member {
                syn::Member::Named(ident) => {
                    matches!(ident.to_string().as_str(), "address" | "addr")
                }
                syn::Member::Unnamed(_) => false,
            },
            _ => false,
        }
    }
}

fn is_str_literal(expr: &Expr) -> bool {
    matches!(expr, Expr::Lit(l) if matches!(l.lit, Lit::Str(_)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    fn run_on_src(src: &str) -> Result<Vec<Finding>, syn::Error> {
        let file = parse_file(src)?;
        Ok(AddressStrEqCheck.run(&file, src))
    }

    #[test]
    fn flags_address_eq_string() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Address, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn check_addr(env: Env, user: Address) {
        if user == "GAAAA" {
            let _ = env;
        }
    }
}
"#,
        )?;
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::Low);
        Ok(())
    }

    #[test]
    fn flags_string_eq_address() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Address, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn check_addr(env: Env, user: Address) {
        if "GAAAA" == user {
            let _ = env;
        }
    }
}
"#,
        )?;
        assert_eq!(hits.len(), 1);
        Ok(())
    }

    #[test]
    fn passes_when_address_eq_address() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Address, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn check_addr(env: Env, user: Address, other: Address) {
        if user == other {
            let _ = env;
        }
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn passes_when_string_eq_string() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn check_str(env: Env) {
        if "hello" == "world" {
            let _ = env;
        }
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn ignores_non_contractimpl() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::Address;

pub struct Contract;

impl Contract {
    pub fn check_addr(user: Address) {
        if user == "GAAAA" {
        }
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }
}
