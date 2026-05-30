#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env, Symbol};

#[contract]
pub struct VulnerableContract;

#[contractimpl]
impl VulnerableContract {
    /// Vulnerable: uses keccak256 for a simple integrity check with no Ethereum requirement.
    pub fn store_hash(env: Env, data: Bytes) {
        // ❌ keccak256 used without any Ethereum-compatibility need; sha256 is sufficient
        let hash: BytesN<32> = env.crypto().keccak256(&data);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "hash"), &hash);
    }
}
