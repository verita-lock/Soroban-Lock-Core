#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct SafeContract;

const KEY: u32 = 0;

#[contractimpl]
impl SafeContract {
    // ✅ Extends TTL after reading to prevent silent expiry
    pub fn get_value(env: Env) -> u32 {
        let v = env.storage().persistent().get(&KEY).unwrap_or(0);
        env.storage().persistent().extend_ttl(&KEY, 1000, 2000);
        v
    }
}
