#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct InitNoEventSafe;

const INITIALIZED: Symbol = symbol_short!("init");

#[contractimpl]
impl InitNoEventSafe {
    /// init with event — should not trigger `init-no-event`.
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.events().publish((INITIALIZED,), (admin,));
    }

    /// initialize with event — should not trigger `init-no-event`.
    pub fn initialize(env: Env, admin: Address, version: u32) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "version"), &version);
        env.events().publish((INITIALIZED,), (admin, version));
    }

    /// setup with event — should not trigger `init-no-event`.
    pub fn setup(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.events().publish((INITIALIZED,), (admin,));
    }

    /// Non-init function without event — should not trigger.
    pub fn update_admin(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        // No event needed for non-init functions
    }

    /// init without storage write — should not trigger.
    pub fn init_no_storage(env: Env) {
        // No storage writes, so no event needed
        env.events().publish((INITIALIZED,), ());
    }
}