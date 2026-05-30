#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct VulnerableContract;

#[contractimpl]
impl VulnerableContract {
    /// Vulnerable: hardcoded Stellar network passphrase in contract logic.
    pub fn get_network(env: Env) -> soroban_sdk::String {
        // ❌ Hardcoded passphrase — fragile across network deployments
        let passphrase = "Test SDF Network ; September 2015";
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "net"), &passphrase);
        soroban_sdk::String::from_str(&env, passphrase)
    }
}
