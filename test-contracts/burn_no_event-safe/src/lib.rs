#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, symbol_short};

#[contract]
pub struct BurnNoEventSafe;

#[contractimpl]
impl BurnNoEventSafe {
    pub fn burn(env: Env, from: Address, amount: i128) {
        env.events().publish((symbol_short!("burn"), from), amount);
        let _ = (from, amount);
        env.storage().persistent().set(&symbol_short!("supply"), &0i128);
    }
}
