#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, symbol_short};

#[contract]
pub struct BurnNoEventVulnerable;

#[contractimpl]
impl BurnNoEventVulnerable {
    pub fn burn(env: Env, from: Address, amount: i128) {
        let _ = (from, amount);
        env.storage().persistent().set(&symbol_short!("supply"), &0i128);
    }
}
