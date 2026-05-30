#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, symbol_short};

#[contract]
pub struct MintNoCapSafe;

#[contractimpl]
impl MintNoCapSafe {
    pub fn mint(env: Env, to: Address, amount: i128) {
        let supply: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("supply"))
            .unwrap_or(0);
        let max_supply: i128 = env
            .storage()
            .persistent()
            .get(&symbol_short!("max_supply"))
            .unwrap_or(0);

        assert!(supply + amount <= max_supply);
        env.storage().persistent().set(&symbol_short!("supply"), &(supply + amount));
        let _ = (to, amount);
    }
}
