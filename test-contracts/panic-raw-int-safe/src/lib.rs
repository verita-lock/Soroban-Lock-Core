#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    InvalidAmount = 1,
    AmountTooLarge = 2,
}

#[contract]
pub struct PanicRawIntSafe;

#[contractimpl]
impl PanicRawIntSafe {
    /// ✅ Uses a typed `#[contracterror]` enum variant — structured error.
    pub fn withdraw(env: Env, amount: i128) {
        if amount <= 0 {
            panic_with_error!(&env, ContractError::InvalidAmount);
        }
    }

    /// ✅ Uses a typed `#[contracterror]` enum variant — structured error.
    pub fn transfer(env: Env, amount: i128) {
        if amount > 1_000_000 {
            panic_with_error!(&env, ContractError::AmountTooLarge);
        }
    }
}
