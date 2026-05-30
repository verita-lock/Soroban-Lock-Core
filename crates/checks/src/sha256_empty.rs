//! Detects `env.crypto().sha256()` or `env.crypto().keccak256()` called on an empty input.
//! Hashing empty data produces a well-known constant digest, which is usually a bug
//! when used as a commitment or identifier.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprArray, ExprMethodCall, ExprPath, ExprReference, File, Lit};

const CHECK_NAME: &str = "sha256-empty";

pub struct Sha256EmptyCheck;

impl Check for Sha256EmptyCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut scan = Sha256EmptyScan {
                fn_name,
                out: &mut out,
            };
            scan.visit_block(&method.block);
        }
        out
    }
}

struct Sha256EmptyScan<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for Sha256EmptyScan<'_> {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_crypto_hash_call(i) && has_empty_argument(i) {
            let line = i.span().start().line;
            self.out.push(Finding {
                check_name: CHECK_NAME.to_string(),
                severity: Severity::Low,
                file_path: String::new(),
                line,
                function_name: self.fn_name.clone(),
                description: format!(
                    "Method `{}` calls `env.crypto().{}` on an empty input. "
                    "Hashing empty data yields a known constant digest and is likely a bug.",
                    self.fn_name,
                    i.method
                ),
            });
        }
        visit::visit_expr_method_call(self, i);
    }
}

fn is_crypto_hash_call(m: &ExprMethodCall) -> bool {
    if m.method != "sha256" && m.method != "keccak256" {
        return false;
    }
    if let Expr::MethodCall(inner) = &*m.receiver {
        if inner.method == "crypto" {
            return true;
        }
    }
    false
}

fn has_empty_argument(m: &ExprMethodCall) -> bool {
    if m.args.len() != 1 {
        return false;
    }
    is_empty_expr(&m.args[0])
}

fn is_empty_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Reference(ExprReference { expr: inner, .. }) => is_empty_expr(inner),
        Expr::MethodCall(call) => is_bytes_empty_constructor(call),
        Expr::Array(ExprArray { elems, .. }) => elems.is_empty(),
        Expr::Lit(lit) => match &lit.lit {
            Lit::ByteStr(bs) => bs.value().is_empty(),
            _ => false,
        },
        _ => false,
    }
}

fn is_bytes_empty_constructor(call: &ExprMethodCall) -> bool {
    if call.args.is_empty() {
        return false;
    }
    if let Expr::Path(ExprPath { path, .. }) = &*call.receiver {
        let receiver = path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        if !receiver.ends_with("Bytes") {
            return false;
        }
    } else {
        return false;
    }

    match call.method.as_str() {
        "new" => call.args.len() == 1,
        "from_slice" => call.args.len() == 2 && is_empty_expr(&call.args[1]),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_file;

    #[test]
    fn flags_sha256_on_bytes_new_empty() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn commit(env: Env) {
        let hash = env.crypto().sha256(&Bytes::new(&env));
    }
}
"#;
        let file = parse_file(code).unwrap();
        let findings = Sha256EmptyCheck.run(&file, code);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Low);
    }

    #[test]
    fn flags_keccak256_on_bytes_from_slice_empty() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn commit(env: Env) {
        let hash = env.crypto().keccak256(&Bytes::from_slice(&env, &[]));
    }
}
"#;
        let file = parse_file(code).unwrap();
        let findings = Sha256EmptyCheck.run(&file, code);
        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn flags_sha256_on_empty_byte_literal() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn commit(env: Env) {
        let hash = env.crypto().sha256(&b"");
    }
}
"#;
        let file = parse_file(code).unwrap();
        let findings = Sha256EmptyCheck.run(&file, code);
        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn allows_sha256_on_non_empty_bytes() {
        let code = r#"
#[contractimpl]
impl MyContract {
    pub fn commit(env: Env) {
        let hash = env.crypto().sha256(&b"data");
    }
}
"#;
        let file = parse_file(code).unwrap();
        let findings = Sha256EmptyCheck.run(&file, code);
        assert!(findings.is_empty());
    }
}
