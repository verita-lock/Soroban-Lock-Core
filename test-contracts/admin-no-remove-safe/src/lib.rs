#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct AdminNoRemoveSafe;

#[contractimpl]
impl AdminNoRemoveSafe {
    /// ✅ Admins can be added and removed.
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

    pub fn remove_admin(env: Env, admin: Address) {
        let current: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("admin"))
            .unwrap();
        current.require_auth();
        let _ = admin;
        env.storage().instance().remove(&symbol_short!("admin"));
    }
}
