#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct SafeContract;

const KEY: u32 = 0;

#[contractimpl]
impl SafeContract {
    // ✅ Only the write entrypoint extends TTL; read-only path does not
    pub fn set_value(env: Env, val: u32) {
        env.storage().instance().extend_ttl(1000, 2000);
        env.storage().instance().set(&KEY, &val);
    }

    pub fn get_value(env: Env) -> u32 {
        env.storage().instance().get(&KEY).unwrap_or(0)
    }
}
