#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct NoAdminSafe;

#[contractimpl]
impl NoAdminSafe {
    pub fn set_admin(env: Env, admin: Address) {
        env.storage().instance().set(&"admin", &admin);
    }

    pub fn update_data(env: Env, value: i32) {
        env.storage().instance().set(&"data", &value);
    }
}
