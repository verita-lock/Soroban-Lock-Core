#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct NoEventsAtAllSafe;

const VALUE_SET: Symbol = symbol_short!("set");
const VALUE_REMOVED: Symbol = symbol_short!("removed");
const INITIALIZED: Symbol = symbol_short!("init");

#[contractimpl]
impl NoEventsAtAllSafe {
    /// Storage set with events — should not trigger `no-events-at-all`.
    pub fn set_value(env: Env, key: Symbol, value: i128) {
        env.storage().persistent().set(&key, &value);
        env.events().publish((VALUE_SET,), (key, value));
    }
    
    /// Storage remove with events — should not trigger `no-events-at-all`.
    pub fn remove_value(env: Env, key: Symbol) {
        env.storage().persistent().remove(&key);
        env.events().publish((VALUE_REMOVED,), (key,));
    }
    
    /// Multiple storage operations with events — should not trigger `no-events-at-all`.
    pub fn init(env: Env, admin: Address, version: u32) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "version"), &version);
        env.events().publish((INITIALIZED,), (admin, version));
    }
    
    /// Function without storage operations — doesn't affect the check.
    pub fn read_value(env: Env, key: Symbol) -> Option<i128> {
        env.storage().persistent().get(&key)
    }
    
    /// Function that only emits events — ensures file has events.
    pub fn log_event(env: Env) {
        env.events().publish((VALUE_SET,), ());
    }
}