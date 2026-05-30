#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct SafeContract;

const KEY: Symbol = symbol_short!("bal");

#[contractimpl]
impl SafeContract {
    // ✅ Errors are handled explicitly via match — the Err arm is not ignored.
    pub fn process(env: Env) {
        match env.storage().instance().get::<Symbol, u32>(&KEY) {
            Some(val) => {
                env.storage().instance().set(&KEY, &(val + 1));
            }
            None => {
                env.storage().instance().set(&KEY, &1u32);
            }
        }
    }

    // ✅ if let Ok with an explicit else branch is also acceptable.
    pub fn process_with_else(env: Env) {
        if let Some(val) = env.storage().instance().get::<Symbol, u32>(&KEY) {
            env.storage().instance().set(&KEY, &(val + 1));
        } else {
            env.storage().instance().set(&KEY, &1u32);
        }
    }
}
