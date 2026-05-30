#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct NoAdminVulnerable;

#[contractimpl]
impl NoAdminVulnerable {
    pub fn update_data(env: Env, value: i32) {
        env.storage().instance().set(&"data", &value);
    }
}
