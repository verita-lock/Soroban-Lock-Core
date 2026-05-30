#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct TryIntoUnwrapSafe;

#[contractimpl]
impl TryIntoUnwrapSafe {
    // ✅ Use unwrap_or with a default value
    pub fn convert_u64_to_u32(env: Env, val: u64) -> u32 {
        val.try_into().unwrap_or(u32::MAX)
    }

    // ✅ Explicit error handling with match
    pub fn convert_multiple(env: Env, a: u64, b: u64, c: u64) -> Result<(u32, u32, u32), &'static str> {
        Ok((
            a.try_into().map_err(|_| "a out of range")?,
            b.try_into().map_err(|_| "b out of range")?,
            c.try_into().map_err(|_| "c out of range")?,
        ))
    }

    // ✅ Check bounds before conversion
    pub fn calculate_safe_amount(env: Env, amount: u128) -> Result<u32, &'static str> {
        const MAX_U32: u128 = u32::MAX as u128;
        if amount > MAX_U32 {
            return Err("amount exceeds u32 max");
        }
        Ok((amount as u32) / 2)
    }

    // ✅ Use unwrap_or_else for computed default
    pub fn safe_convert_with_fallback(env: Env, val: u64) -> u32 {
        val.try_into().unwrap_or_else(|_| 1)
    }
}
