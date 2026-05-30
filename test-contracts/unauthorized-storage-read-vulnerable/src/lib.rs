//! Vulnerable contract that triggers the unauthorized-storage-read check.

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct UnauthorizedStorageReadVulnerable;

const KEY: Symbol = symbol_short!("key");

#[contractimpl]
impl UnauthorizedStorageReadVulnerable {
    pub fn read_data(env: Env) {
        // No require_auth call
        let val = env.storage().persistent().get(&KEY);
    }
}
