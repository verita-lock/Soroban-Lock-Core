#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct HashInLoopSafe;

#[contractimpl]
impl HashInLoopSafe {
    // ✅ Hash computed once outside the loop
    pub fn process(env: Env, count: u32) {
        let h = env.crypto().sha256(&());
        for i in 0..count {
            let _ = (i, &h);
        }
    }
}
