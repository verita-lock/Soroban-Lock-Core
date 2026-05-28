//! Safe contract that does not trigger the storage-has-get-race check.

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct StorageHasGetRaceSafe;

const KEY: Symbol = symbol_short!("key");

#[contractimpl]
impl StorageHasGetRaceSafe {
    pub fn get_directly(env: Env) {
        // No race condition: get directly
        let val = env.storage().persistent().get(&KEY);
    }
}
