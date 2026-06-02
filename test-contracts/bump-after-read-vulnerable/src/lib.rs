#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct VulnerableContract;

const KEY: u32 = 0;

#[contractimpl]
impl VulnerableContract {
    // ❌ Reads persistent entry but never calls extend_ttl — TTL decays on every access
    pub fn get_value(env: Env) -> u32 {
        env.storage().persistent().get(&KEY).unwrap_or(0)
    }
}
