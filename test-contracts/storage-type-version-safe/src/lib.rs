//! Safe contract that does not trigger the storage-type-version check.

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct StorageTypeVersionSafe;

const KEY: Symbol = symbol_short!("key");

#[contractimpl]
impl StorageTypeVersionSafe {
    pub fn single_storage(env: Env) {
        // Only uses persistent storage
        env.storage().persistent().set(&KEY, &1);
        env.storage().persistent().set(&KEY, &2);
    }
}
