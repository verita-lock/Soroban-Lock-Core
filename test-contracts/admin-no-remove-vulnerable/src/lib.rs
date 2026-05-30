#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct AdminNoRemoveVulnerable;

#[contractimpl]
impl AdminNoRemoveVulnerable {
    /// ❌ Admins can be added but never removed — a compromised key is permanent.
    pub fn add_admin(env: Env, new_admin: Address) {
        let current: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("admin"))
            .unwrap();
        current.require_auth();
        env.storage()
            .instance()
            .set(&symbol_short!("admin"), &new_admin);
    }
}
