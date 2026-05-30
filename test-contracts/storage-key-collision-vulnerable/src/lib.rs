//! Vulnerable contract that triggers the storage-key-collision check.

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct StorageKeyCollisionVulnerable;

const OWNER: Symbol = symbol_short!("owner");
const OWNER_ADDR: Symbol = symbol_short!("owner_addr");
const OWNER_ADDRESS: Symbol = symbol_short!("owner_address");

#[contractimpl]
impl StorageKeyCollisionVulnerable {
    pub fn store_owner(env: Env, owner: soroban_sdk::Address) {
        env.storage().persistent().set(&OWNER, &owner);
        env.storage().persistent().set(&OWNER_ADDR, &owner);
        env.storage().persistent().set(&OWNER_ADDRESS, &owner);
    }
}
