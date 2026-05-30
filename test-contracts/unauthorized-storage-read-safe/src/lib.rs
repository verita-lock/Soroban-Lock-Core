//! Safe contract that does not trigger the unauthorized-storage-read check.

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct UnauthorizedStorageReadSafe;

const KEY: Symbol = symbol_short!("key");

#[contractimpl]
impl UnauthorizedStorageReadSafe {
    pub fn read_data(env: Env) {
        env.require_auth();
        let val = env.storage().persistent().get(&KEY);
    }
}
