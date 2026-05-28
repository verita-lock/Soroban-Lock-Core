#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct AuthTempStorageSafe;

#[contractimpl]
impl AuthTempStorageSafe {
    /// Reads admin from persistent storage (not temporary).
    pub fn safe_persistent_auth(env: Env) {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&symbol_short!("admin"))
            .unwrap();
        env.require_auth(&admin);
        env.storage()
            .instance()
            .set(&symbol_short!("data"), &42u32);
    }
}
