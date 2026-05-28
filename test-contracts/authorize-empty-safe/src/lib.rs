#![no_std]
use soroban_sdk::{contract, contractimpl, Env, InvokeContract};

#[contract]
pub struct AuthorizeEmptySafe;

#[contractimpl]
impl AuthorizeEmptySafe {
    /// Calls authorize_as_current_contract with proper invocation vector.
    pub fn safe_with_invocation(env: Env, invocation: InvokeContract) {
        env.authorize_as_current_contract(&[invocation]);
    }
}
