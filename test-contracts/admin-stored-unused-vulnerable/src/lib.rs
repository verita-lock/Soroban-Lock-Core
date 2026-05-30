#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct AdminStoredUnusedVulnerable;

#[contractimpl]
impl AdminStoredUnusedVulnerable {
    /// ❌ Admin is stored but never used in require_auth — cosmetic only.
    pub fn init(env: Env, admin: Address) {
        env.storage()
            .instance()
            .set(&symbol_short!("admin"), &admin);
    }

    pub fn do_thing(env: Env) {
        // No admin check — anyone can call this
        env.storage()
            .instance()
            .set(&symbol_short!("val"), &42u32);
    }
}
