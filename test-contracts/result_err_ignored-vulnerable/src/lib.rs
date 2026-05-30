#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct VulnerableContract;

const KEY: Symbol = symbol_short!("bal");

#[contractimpl]
impl VulnerableContract {
    // ❌ Error from get() is silently ignored — if the key is missing the
    //    branch is simply skipped with no indication of failure.
    pub fn process(env: Env) {
        if let Ok(val) = env.storage().instance().get::<Symbol, u32>(&KEY) {
            env.storage().instance().set(&KEY, &(val + 1));
        }
    }
}
