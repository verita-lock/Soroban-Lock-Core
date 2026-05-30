#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct Sha256EmptySafe;

#[contractimpl]
impl Sha256EmptySafe {
    pub fn commit(env: Env, data: Bytes) {
        let _hash = env.crypto().sha256(&data);
    }
}
