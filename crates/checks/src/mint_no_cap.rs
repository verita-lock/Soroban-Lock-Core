//! Mint functions that do not enforce a declared maximum supply cap.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{BinOp, Expr, ExprBinary, ExprMethodCall, File};

const CHECK_NAME: &str = "mint-no-cap";

const SUPPLY_HINTS: &[&str] = &["supply", "total", "minted", "cap", "max"];

pub struct MintNoCapCheck;

impl Check for MintNoCapCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            if method.sig.ident != "mint" || !matches!(method.vis, syn::Visibility::Public(_)) {
                continue;
            }

            let mut scan = MintScan::default();
            scan.visit_block(&method.block);

            if scan.supply_get && scan.storage_write && !scan.cap_check {
                out.push(Finding {
                    check_name: CHECK_NAME.to_string(),
                    severity: Severity::Medium,
                    file_path: String::new(),
                    line: scan.write_line.unwrap_or_else(|| method.sig.ident.span().start().line),
                    function_name: "mint".to_string(),
                    description: "Method `mint` updates a supply value from storage but does not enforce a `max_supply` or similar cap before writing. This can allow infinite inflation.".to_string(),
                });
            }
        }
        out
    }
}

#[derive(Default)]
struct MintScan {
    supply_get: bool,
    storage_write: bool,
    cap_check: bool,
    write_line: Option<usize>,
}

impl<'ast> Visit<'ast> for MintScan {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if receiver_chain_contains_storage(&i.receiver) {
            match i.method.to_string().as_str() {
                "get" | "get_unchecked" => {
                    if i.args.iter().any(|arg| expr_contains_supply_hint(arg)) {
                        self.supply_get = true;
                    }
                }
                "set" => {
                    self.storage_write = true;
                    if self.write_line.is_none() {
                        self.write_line = Some(i.span().start().line);
                    }
                }
                _ => {}
            }
        }
        visit::visit_expr_method_call(self, i);
    }

    fn visit_expr_binary(&mut self, i: &'ast ExprBinary) {
        if matches!(i.op, BinOp::Le(_) | BinOp::Lt(_)) {
            self.cap_check = true;
        }
        visit::visit_expr_binary(self, i);
    }
}

fn receiver_chain_contains_storage(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            if m.method == "storage" {
                return true;
            }
            receiver_chain_contains_storage(&m.receiver)
        }
        Expr::Field(f) => receiver_chain_contains_storage(&f.base),
        _ => false,
    }
}

fn expr_contains_supply_hint(expr: &Expr) -> bool {
    let text = expr_to_text(expr).to_lowercase();
    SUPPLY_HINTS.iter().any(|hint| text.contains(hint))
}

fn expr_to_text(expr: &Expr) -> String {
    match expr {
        Expr::Path(p) => p.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("_"),
        Expr::Macro(m) => m.mac.tokens.to_string(),
        Expr::Lit(l) => match &l.lit {
            syn::Lit::Str(s) => s.value(),
            _ => String::new(),
        },
        Expr::Reference(r) => expr_to_text(&r.expr),
        Expr::Call(c) => {
            let mut s = expr_to_text(&c.func);
            for arg in &c.args {
                s.push('_');
                s.push_str(&expr_to_text(arg));
            }
            s
        }
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    fn run(src: &str) -> Vec<Finding> {
        MintNoCapCheck.run(&parse_file(src).unwrap(), src)
    }

    #[test]
    fn flags_mint_without_cap_check() {
        let hits = run(r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn mint(env: Env, to: Address, amount: i128) {
        let supply: i128 = env.storage().persistent().get(&symbol_short!("supply")).unwrap_or(0);
        env.storage().persistent().set(&symbol_short!("supply"), &(supply + amount));
    }
}
"#);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].function_name, "mint");
        assert_eq!(hits[0].severity, Severity::Medium);
    }

    #[test]
    fn passes_when_cap_guard_present() {
        let hits = run(r#"
use soroban_sdk::{contractimpl, symbol_short, Address, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn mint(env: Env, to: Address, amount: i128) {
        let supply: i128 = env.storage().persistent().get(&symbol_short!("supply")).unwrap_or(0);
        let max_supply: i128 = env.storage().persistent().get(&symbol_short!("max_supply")).unwrap();
        assert!(supply + amount <= max_supply);
        env.storage().persistent().set(&symbol_short!("supply"), &(supply + amount));
    }
}
"#);
        assert!(hits.is_empty());
    }
}
