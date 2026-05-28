//! Vulnerable contract that triggers the storage-has-get-race check.

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct StorageHasGetRaceVulnerable;

const KEY: Symbol = symbol_short!("key");

#[contractimpl]
impl StorageHasGetRaceVulnerable {
    pub fn has_then_get(env: Env) {
        // Race condition: has then get on same key
        if env.storage().persistent().has(&KEY) {
            let val = env.storage().persistent().get(&KEY);
        }
    }
}
