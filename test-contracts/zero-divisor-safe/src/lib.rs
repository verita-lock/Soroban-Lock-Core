#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct ZeroDivisorSafe;

#[contractimpl]
impl ZeroDivisorSafe {
    /// Division by `rate` guarded by an `assert!` — should not trigger `zero-divisor`.
    pub fn quote(_env: Env, _amount: i128, rate: i128) -> i128 {
        assert!(rate != 0, "rate must be nonzero");
        _amount / rate
    }
}
