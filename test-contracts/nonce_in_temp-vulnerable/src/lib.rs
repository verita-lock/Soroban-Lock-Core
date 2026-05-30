#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct NonceInTempVulnerable;

const NONCE: Symbol = symbol_short!("nonce");

#[contractimpl]
impl NonceInTempVulnerable {
    /// ❌ Nonce stored in temporary storage — expires with TTL, enabling replay attacks.
    pub fn execute(env: Env, n: u64) {
        let current: u64 = env.storage().temporary().get(&NONCE).unwrap_or(0);
        assert!(n > current, "replay");
        env.storage().temporary().set(&NONCE, &n);
    }
}
