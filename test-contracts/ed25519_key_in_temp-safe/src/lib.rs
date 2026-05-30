#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct Ed25519KeyInTempSafe;

#[contractimpl]
impl Ed25519KeyInTempSafe {
    pub fn verify(env: Env, pubkey: Bytes, msg: Bytes, sig: Bytes) {
        let _ok = env.crypto().ed25519_verify(&pubkey, &msg, &sig);
    }
}
