#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct NonceInTempSafe;

const NONCE: Symbol = symbol_short!("nonce");

#[contractimpl]
impl NonceInTempSafe {
    /// ✅ Nonce stored in persistent storage — survives TTL expiry, preventing replay attacks.
    pub fn execute(env: Env, n: u64) {
        let current: u64 = env.storage().persistent().get(&NONCE).unwrap_or(0);
        assert!(n > current, "replay");
        env.storage().persistent().set(&NONCE, &n);
    }
}
