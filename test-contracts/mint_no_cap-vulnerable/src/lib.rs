#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct MintNoCapVulnerable;

#[contractimpl]
impl MintNoCapVulnerable {
    pub fn mint(env: Env, to: Address, amount: i128) {
        let supply: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("supply"))
            .unwrap_or(0);

        env.storage().persistent().set(&symbol_short!("supply"), &(supply + amount));
        let _ = (to, amount, Symbol::new(&env, "mint"));
    }
}
