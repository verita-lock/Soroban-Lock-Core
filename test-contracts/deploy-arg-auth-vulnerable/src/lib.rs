#![no_std]
use soroban_sdk::{contract, contractimpl, BytesN, Env};

#[contract]
pub struct DeployArgAuthVulnerable;

#[contractimpl]
impl DeployArgAuthVulnerable {
    /// Calls require_auth but deploy args are user-supplied parameters.
    pub fn vulnerable_deploy(env: Env, wasm_hash: BytesN<32>, salt: u64) {
        env.require_auth();
        // Vulnerable: auth doesn't bind to wasm_hash and salt
        env.deployer().deploy(wasm_hash, salt);
    }
}
