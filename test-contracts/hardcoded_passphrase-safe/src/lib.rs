#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Symbol};

#[contract]
pub struct SafeContract;

#[contractimpl]
impl SafeContract {
    /// Safe: network passphrase passed as a parameter, not hardcoded.
    pub fn get_network(env: Env, passphrase: String) -> String {
        // ✅ Passphrase supplied by caller — works across testnet/mainnet
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "net"), &passphrase);
        passphrase
    }
}
