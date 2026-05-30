//! Unnecessary conversion of a Soroban `Map` to `Vec<(Symbol, Val)>` via
//! `.to_vec()` or `.into_vec()` when direct `Map` operations should be used.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, File};

const CHECK_NAME: &str = "vec-map-tuple-convert";

pub struct VecMapTupleConvertCheck;

impl Check for VecMapTupleConvertCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut scan = MapToVecScan {
                fn_name: fn_name.clone(),
                out: &mut out,
                map_locals: Vec::new(),
            };
            scan.visit_block(&method.block);
        }
        out
    }
}

struct MapToVecScan<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
    /// Local variable names that are typed or named as Map.
    map_locals: Vec<String>,
}

fn is_map_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Path(p) => {
            let s = p
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::")
                .to_lowercase();
            s.contains("map")
        }
        Expr::MethodCall(m) => {
            // map.clone(), map.get(...), etc.
            is_map_expr(&m.receiver)
        }
        _ => false,
    }
}

fn type_is_map(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(p) => {
            if let Some(seg) = p.path.segments.last() {
                seg.ident == "Map"
            } else {
                false
            }
        }
        _ => false,
    }
}

impl<'ast> Visit<'ast> for MapToVecScan<'_> {
    fn visit_local(&mut self, i: &'ast syn::Local) {
        // Track `let map: Map<...> = ...` or `let map = ...` where name contains "map"
        let is_map_typed = if let syn::Pat::Type(pt) = &i.pat {
            type_is_map(&pt.ty)
        } else {
            false
        };
        let ident = match &i.pat {
            syn::Pat::Ident(p) => Some(p.ident.to_string()),
            syn::Pat::Type(pt) => {
                if let syn::Pat::Ident(p) = &*pt.pat {
                    Some(p.ident.to_string())
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(name) = ident {
            let lower = name.to_lowercase();
            if is_map_typed || lower.contains("map") {
                self.map_locals.push(name);
            }
        }
        visit::visit_local(self, i);
    }

    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        let method = i.method.to_string();
        if method == "to_vec" || method == "into_vec" {
            // Check if receiver is a Map variable or Map-typed expression
            let receiver_is_map = is_map_expr(&i.receiver) || {
                if let Expr::Path(p) = &*i.receiver {
                    if let Some(seg) = p.path.segments.last() {
                        self.map_locals.contains(&seg.ident.to_string())
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            if receiver_is_map {
                self.out.push(Finding {
                    check_name: CHECK_NAME.to_string(),
                    severity: Severity::Low,
                    file_path: String::new(),
                    line: i.span().start().line,
                    function_name: self.fn_name.clone(),
                    description: format!(
                        "Function `{}` calls `.{method}()` on a Map, converting it to a \
                         Vec of tuples. Use direct Map operations (`.get()`, `.set()`, \
                         `.keys()`) instead to avoid unnecessary compute overhead.",
                        self.fn_name
                    ),
                });
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

    #[test]
    fn flags_map_to_vec() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Map, Symbol, Val, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn process(env: Env, map: Map<Symbol, Val>) {
        let v = map.to_vec();
    }
}
"#,
        )?;
        let hits = VecMapTupleConvertCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::Low);
        assert_eq!(hits[0].function_name, "process");
        Ok(())
    }

    #[test]
    fn flags_map_into_vec() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Map, Symbol, Val, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn process(env: Env, my_map: Map<Symbol, Val>) {
        let v = my_map.into_vec();
    }
}
"#,
        )?;
        let hits = VecMapTupleConvertCheck.run(&file, "");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::Low);
        Ok(())
    }

    #[test]
    fn passes_when_using_map_get() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Map, Symbol, Val, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn process(env: Env, map: Map<Symbol, Val>, key: Symbol) {
        let _ = map.get(key);
    }
}
"#,
        )?;
        let hits = VecMapTupleConvertCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }

    #[test]
    fn ignores_vec_to_vec() -> Result<(), syn::Error> {
        let file = parse_file(
            r#"
use soroban_sdk::{contractimpl, Vec, Env};
pub struct C;
#[contractimpl]
impl C {
    pub fn process(env: Env, items: Vec<u32>) {
        let v = items.to_vec();
    }
}
"#,
        )?;
        let hits = VecMapTupleConvertCheck.run(&file, "");
        assert!(hits.is_empty());
        Ok(())
    }
}
