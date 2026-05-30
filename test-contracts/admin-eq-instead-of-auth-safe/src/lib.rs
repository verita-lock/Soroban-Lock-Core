#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct AdminEqInsteadOfAuthSafe;

#[contractimpl]
impl AdminEqInsteadOfAuthSafe {
    pub fn init(env: Env, admin: Address) {
        env.storage()
            .instance()
            .set(&symbol_short!("admin"), &admin);
    }

    /// ✅ Uses require_auth on the stored admin — proper host-level verification.
    pub fn protected(env: Env) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("admin"))
            .unwrap();
        admin.require_auth();
        env.storage()
            .instance()
            .set(&symbol_short!("val"), &1u32);
    }
}
