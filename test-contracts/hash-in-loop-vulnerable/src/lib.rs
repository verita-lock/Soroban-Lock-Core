#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct HashInLoopVulnerable;

#[contractimpl]
impl HashInLoopVulnerable {
    // ❌ sha256 called inside a for loop — wastes compute budget
    pub fn process_sha256(env: Env, count: u32) {
        for i in 0..count {
            let _ = (i, env.crypto().sha256(&()));
        }
    }

    // ❌ keccak256 called inside a while loop
    pub fn process_keccak256(env: Env) {
        let mut i = 0u32;
        while i < 10 {
            let _ = env.crypto().keccak256(&());
            i += 1;
        }
    }
}
