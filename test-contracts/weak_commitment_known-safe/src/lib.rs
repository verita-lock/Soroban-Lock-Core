#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, Symbol};

#[contract]
pub struct SafeContract;

#[contractimpl]
impl SafeContract {
    /// Safe: sha256(address + secret_param + nonce) — secret is passed in, not stored.
    pub fn commit(env: Env, user: Address, secret: Bytes, nonce: Bytes) {
        // ✅ Secret and nonce are caller-supplied, not reconstructable from storage alone
        let hash = env.crypto().sha256(&(user, secret, nonce));
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "commit"), &hash);
    }
}
