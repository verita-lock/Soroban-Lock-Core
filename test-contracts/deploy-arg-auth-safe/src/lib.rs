#![no_std]
use soroban_sdk::{contract, contractimpl, BytesN, Env};

#[contract]
pub struct DeployArgAuthSafe;

#[contractimpl]
impl DeployArgAuthSafe {
    /// Uses require_auth_for_args to bind auth to deploy parameters.
    pub fn safe_deploy(env: Env, wasm_hash: BytesN<32>, salt: u64) {
        env.require_auth_for_args((wasm_hash, salt));
        env.deployer().deploy(wasm_hash, salt);
    }
}
