#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct AddressCmpVulnerable;

#[contractimpl]
impl AddressCmpVulnerable {
    /// Compares caller address instead of using require_auth.
    pub fn vulnerable_cmp(env: Env, caller: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("admin"))
            .unwrap();
        if caller == admin {
            // Vulnerable: comparison instead of require_auth
            env.storage()
                .instance()
                .set(&symbol_short!("data"), &42u32);
        }
    }
}
