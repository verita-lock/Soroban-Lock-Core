#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct AuthorizeEmptyVulnerable;

#[contractimpl]
impl AuthorizeEmptyVulnerable {
    /// Calls authorize_as_current_contract with empty invocation vector.
    pub fn vulnerable_empty_auth(env: Env) {
        env.authorize_as_current_contract(&[]);
    }
}
