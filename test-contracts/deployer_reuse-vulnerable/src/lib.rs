#![no_std]
use soroban_sdk::{contract, contractimpl, BytesN, Env};

#[contract]
pub struct DeployerReuseVulnerable;

#[contractimpl]
impl DeployerReuseVulnerable {
    /// ❌ Caches env.deployer() once and calls .deploy() on it twice.
    /// The stale deployer reference may produce incorrect results on the second call.
    pub fn deploy_two(env: Env, wasm_a: BytesN<32>, wasm_b: BytesN<32>) {
        let deployer = env.deployer();
        deployer.deploy(wasm_a, &[]);
        deployer.deploy(wasm_b, &[]);
    }
}
