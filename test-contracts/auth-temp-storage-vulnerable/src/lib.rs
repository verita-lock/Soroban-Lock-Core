#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct AuthTempStorageVulnerable;

#[contractimpl]
impl AuthTempStorageVulnerable {
    /// Reads admin from temporary storage and uses it in require_auth.
    pub fn vulnerable_temp_auth(env: Env) {
        let admin: Address = env
            .storage()
            .temporary()
            .get(&symbol_short!("admin"))
            .unwrap();
        // Vulnerable: admin may have expired from temporary storage
        env.require_auth(&admin);
        env.storage()
            .instance()
            .set(&symbol_short!("data"), &42u32);
    }
}
