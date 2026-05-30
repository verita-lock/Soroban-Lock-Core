#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // ❌ expect message contains `{}` — leaks format specifier on-chain
    pub fn get_balance(env: Env) -> i128 {
        let balance_key = symbol_short!("bal");
        env.storage()
            .persistent()
            .get(&balance_key)
            .expect("failed to read balance: {}")
    }

    // ❌ expect message contains the key variable name — leaks internal key name
    pub fn get_owner(env: Env) -> i128 {
        let owner_key = symbol_short!("owner");
        env.storage()
            .persistent()
            .get(&owner_key)
            .expect("owner_key not found in storage")
    }
}
