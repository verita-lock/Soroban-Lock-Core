#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct PanicRawIntVulnerable;

#[contractimpl]
impl PanicRawIntVulnerable {
    /// ❌ Raw integer — opaque error code, should trigger `panic-raw-int` (Low).
    pub fn withdraw(env: Env, amount: i128) {
        if amount <= 0 {
            panic_with_error!(&env, 1);
        }
    }

    /// ❌ Another raw integer literal.
    pub fn transfer(env: Env, amount: i128) {
        if amount > 1_000_000 {
            panic_with_error!(&env, 42);
        }
    }
}
