#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct ZeroDivisorVulnerable;

#[contractimpl]
impl ZeroDivisorVulnerable {
    /// Division by `rate` with no zero guard — should trigger `zero-divisor` (High).
    pub fn quote(_env: Env, _amount: i128, rate: i128) -> i128 {
        _amount / rate
    }
}
