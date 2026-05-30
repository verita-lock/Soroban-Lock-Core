#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // ✅ Generic message — no key names or format specifiers exposed
    pub fn get_balance(env: Env) -> i128 {
        let balance_key = symbol_short!("bal");
        env.storage()
            .persistent()
            .get(&balance_key)
            .expect("storage read failed")
    }

    // ✅ Using unwrap_or avoids expect entirely
    pub fn get_owner(env: Env) -> i128 {
        let owner_key = symbol_short!("owner");
        env.storage()
            .persistent()
            .get(&owner_key)
            .unwrap_or(0)
    }
}
