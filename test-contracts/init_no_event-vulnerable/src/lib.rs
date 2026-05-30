#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct InitNoEventVulnerable;

#[contractimpl]
impl InitNoEventVulnerable {
    /// init without event — should trigger `init-no-event` (Low).
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
    }

    /// initialize without event — should trigger `init-no-event` (Low).
    pub fn initialize(env: Env, admin: Address, version: u32) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "version"), &version);
    }

    /// setup without event — should trigger `init-no-event` (Low).
    pub fn setup(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
    }

    /// Private init — should not trigger.
    fn private_init(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
    }

    /// Non-init function with storage write — should not trigger.
    pub fn update_admin(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
    }

    /// init without storage write — should not trigger.
    pub fn init_no_storage(_env: Env) {
        // No storage writes
    }
}