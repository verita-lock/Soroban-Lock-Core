#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct NoEventsAtAllVulnerable;

#[contractimpl]
impl NoEventsAtAllVulnerable {
    /// Storage set without events — should trigger `no-events-at-all` (Medium).
    pub fn set_value(env: Env, key: Symbol, value: i128) {
        env.storage().persistent().set(&key, &value);
    }
    
    /// Storage remove without events — should trigger `no-events-at-all` (Medium).
    pub fn remove_value(env: Env, key: Symbol) {
        env.storage().persistent().remove(&key);
    }
    
    /// Multiple storage operations without events — should trigger `no-events-at-all` (Medium).
    pub fn init(env: Env, admin: Address, version: u32) {
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "version"), &version);
    }
    
    /// Function without storage operations — doesn't affect the check.
    pub fn read_value(env: Env, key: Symbol) -> Option<i128> {
        env.storage().persistent().get(&key)
    }
}