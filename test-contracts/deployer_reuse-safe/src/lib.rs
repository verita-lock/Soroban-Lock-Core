#![no_std]
use soroban_sdk::{contract, contractimpl, BytesN, Env};

#[contract]
pub struct DeployerReuseSafe;

#[contractimpl]
impl DeployerReuseSafe {
    /// ✅ Each deploy() call uses a fresh env.deployer() — no stale reference.
    pub fn deploy_two(env: Env, wasm_a: BytesN<32>, wasm_b: BytesN<32>) {
        env.deployer().deploy(wasm_a, &[]);
        env.deployer().deploy(wasm_b, &[]);
    }
}
