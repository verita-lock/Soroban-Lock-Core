#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct ResultNonExhaustiveVulnerable;

#[contractimpl]
impl ResultNonExhaustiveVulnerable {
    pub fn run(_env: Env) -> i128 {
        let result: Result<i128, ()> = Err(());
        match result {
            Ok(v) => v
        }
    }
}
