#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct AdminEqInsteadOfAuthVulnerable;

#[contractimpl]
impl AdminEqInsteadOfAuthVulnerable {
    pub fn init(env: Env, admin: Address) {
        env.storage()
            .instance()
            .set(&symbol_short!("admin"), &admin);
    }

    /// ❌ Compares caller == admin with == instead of require_auth.
    /// Bypasses host-level signature verification.
    pub fn protected(env: Env, caller: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("admin"))
            .unwrap();
        if caller == admin {
            env.storage()
                .instance()
                .set(&symbol_short!("val"), &1u32);
        }
    }
}
