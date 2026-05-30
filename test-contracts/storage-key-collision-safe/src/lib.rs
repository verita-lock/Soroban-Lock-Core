//! Safe contract that does not trigger the storage-key-collision check.

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct StorageKeyCollisionSafe;

const OWNER: Symbol = symbol_short!("owner");
const BALANCE: Symbol = symbol_short!("balance");
const ALLOWANCE: Symbol = symbol_short!("allowance");

#[contractimpl]
impl StorageKeyCollisionSafe {
    pub fn store_data(env: Env, owner: soroban_sdk::Address, balance: i128, allowance: i128) {
        env.storage().persistent().set(&OWNER, &owner);
        env.storage().persistent().set(&BALANCE, &balance);
        env.storage().persistent().set(&ALLOWANCE, &allowance);
    }
}
