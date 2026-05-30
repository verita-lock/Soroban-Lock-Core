//! Vulnerable contract that triggers the storage-type-version check.

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct StorageTypeVersionVulnerable;

const KEY: Symbol = symbol_short!("key");

#[contractimpl]
impl StorageTypeVersionVulnerable {
    pub fn mixed_storage(env: Env) {
        // Uses both persistent and instance storage
        env.storage().persistent().set(&KEY, &1);
        env.storage().instance().set(&KEY, &2);
    }
}
