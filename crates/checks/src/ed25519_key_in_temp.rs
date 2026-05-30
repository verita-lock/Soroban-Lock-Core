//! Detects `env.crypto().ed25519_verify(...)` where the public key is read from
//! temporary storage. Temporary storage entries expire, so signature verification
//! may later use a default or zero key.

use crate::util::contractimpl_functions;
use crate::{Check, Finding, Severity};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprField, ExprMethodCall, ExprPath, ExprReference, File};

const CHECK_NAME: &str = "ed25519-key-in-temp";

pub struct Ed25519KeyInTempCheck;

impl Check for Ed25519KeyInTempCheck {
    fn name(&self) -> &str {
        CHECK_NAME
    }

    fn run(&self, file: &File, _source: &str) -> Vec<Finding> {
        let mut out = Vec::new();
        for method in contractimpl_functions(file) {
            let fn_name = method.sig.ident.to_string();
            let mut visitor = Ed25519KeyInTempVisitor {
                fn_name,
                out: &mut out,
            };
            visitor.visit_block(&method.block);
        }
        out
    }
}

struct Ed25519KeyInTempVisitor<'a> {
    fn_name: String,
    out: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for Ed25519KeyInTempVisitor<'_> {
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        if is_ed25519_verify_call(i) && has_temporary_pubkey_arg(i) {
            let line = i.span().start().line;
            self.out.push(Finding {
                check_name: CHECK_NAME.to_string(),
                severity: Severity::High,
                file_path: String::new(),
                line,
                function_name: self.fn_name.clone(),
                description: format!(
                    "Method `{}` calls `env.crypto().ed25519_verify(...)` with a public key read from temporary storage. Temporary storage entries expire, which may cause verification to use a default key.",
                    self.fn_name
                ),
            });
        }
        visit::visit_expr_method_call(self, i);
    }
}

fn is_ed25519_verify_call(m: &ExprMethodCall) -> bool {
    if m.method != "ed25519_verify" {
        return false;
    }
    receiver_chain_is_crypto(&m.receiver)
}

fn has_temporary_pubkey_arg(m: &ExprMethodCall) -> bool {
    if m.args.is_empty() {
        return false;
    }
    expr_is_temporary_get(&m.args[0])
}

fn expr_is_temporary_get(expr: &Expr) -> bool {
    match expr {
        Expr::Reference(ExprReference { expr: inner, .. }) => expr_is_temporary_get(inner),
        Expr::MethodCall(call) => is_temporary_get(call),
        Expr::Field(ExprField { base, .. }) => expr_is_temporary_get(base),
        _ => false,
    }
}

fn is_temporary_get(call: &ExprMethodCall) -> bool {
    if call.method != "get" {
        return false;
    }
    receiver_chain_contains_temporary(&call.receiver)
}

fn receiver_chain_contains_temporary(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            if m.method == "temporary" {
                return receiver_chain_contains_storage(&m.receiver);
            }
            receiver_chain_contains_temporary(&m.receiver)
        }
        Expr::Field(f) => receiver_chain_contains_temporary(&f.base),
        _ => false,
    }
}

fn receiver_chain_contains_storage(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(m) => {
            if m.method == "storage" {
                return receiver_chain_is_env(&m.receiver);
            }
            receiver_chain_contains_storage(&m.receiver)
        }
        Expr::Field(f) => receiver_chain_contains_storage(&f.base),
        _ => false,
    }
}

fn receiver_chain_is_crypto(expr: &Expr) -> bool {
    match expr {
        Expr::MethodCall(method) => {
            if method.method == "crypto" {
                return receiver_chain_is_env(&method.receiver);
            }
            receiver_chain_is_crypto(&method.receiver)
        }
        Expr::Field(f) => receiver_chain_is_crypto(&f.base),
        Expr::Path(path) => path.path.is_ident("env"),
        _ => false,
    }
}

fn receiver_chain_is_env(expr: &Expr) -> bool {
    matches!(expr, Expr::Path(path) if path.path.is_ident("env"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;
    use syn::parse_file;

    fn run_on_src(src: &str) -> Result<Vec<Finding>, syn::Error> {
        let file = parse_file(src)?;
        Ok(Ed25519KeyInTempCheck.run(&file, src))
    }

    #[test]
    fn flags_ed25519_verify_with_temp_pubkey() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn verify(env: Env, msg: Bytes, sig: Bytes) {
        let pubkey = env.storage().temporary().get(&"pubkey");
        env.crypto().ed25519_verify(&pubkey, &msg, &sig);
    }
}
"#,
        )?;
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].severity, Severity::High);
        Ok(())
    }

    #[test]
    fn does_not_flag_ed25519_verify_with_persistent_pubkey() -> Result<(), syn::Error> {
        let hits = run_on_src(
            r#"
use soroban_sdk::{contractimpl, Env};

pub struct C;

#[contractimpl]
impl C {
    pub fn verify(env: Env, msg: Bytes, sig: Bytes) {
        let pubkey = env.storage().persistent().get(&"pubkey");
        env.crypto().ed25519_verify(&pubkey, &msg, &sig);
    }
}
"#,
        )?;
        assert!(hits.is_empty());
        Ok(())
    }
}
