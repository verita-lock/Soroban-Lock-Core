#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct AdminStoredUnusedSafe;

#[contractimpl]
impl AdminStoredUnusedSafe {
    pub fn init(env: Env, admin: Address) {
        env.storage()
            .instance()
            .set(&symbol_short!("admin"), &admin);
    }

    /// ✅ Admin is read from storage and used in require_auth.
    pub fn do_thing(env: Env) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("admin"))
            .unwrap();
        admin.require_auth();
        env.storage()
            .instance()
            .set(&symbol_short!("val"), &42u32);
    }
}
