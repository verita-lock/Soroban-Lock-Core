#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env, Symbol};

#[contract]
pub struct SafeContract;

#[contractimpl]
impl SafeContract {
    /// Safe: uses sha256 for integrity check — appropriate for Soroban contracts.
    pub fn store_hash(env: Env, data: Bytes) {
        // ✅ sha256 is the correct choice for non-Ethereum integrity checks
        let hash: BytesN<32> = env.crypto().sha256(&data);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "hash"), &hash);
    }
}
