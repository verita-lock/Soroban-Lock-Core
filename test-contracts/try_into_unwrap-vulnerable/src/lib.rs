#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct TryIntoUnwrapVulnerable;

#[contractimpl]
impl TryIntoUnwrapVulnerable {
    // ❌ try_into() can fail for out-of-range values; unwrap() panics
    pub fn convert_u64_to_u32(env: Env, val: u64) -> u32 {
        val.try_into().unwrap()
    }

    // ❌ Multiple try_into().unwrap() calls without error handling
    pub fn convert_multiple(env: Env, a: u64, b: u64, c: u64) -> (u32, u32, u32) {
        (
            a.try_into().unwrap(),
            b.try_into().unwrap(),
            c.try_into().unwrap(),
        )
    }

    // ❌ try_into().unwrap() in arithmetic operation
    pub fn calculate_safe_amount(env: Env, amount: u128) -> u32 {
        let converted: u32 = amount.try_into().unwrap();
        converted / 2
    }
}
