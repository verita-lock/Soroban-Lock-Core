#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct Ed25519KeyInTempVulnerable;

#[contractimpl]
impl Ed25519KeyInTempVulnerable {
    pub fn verify(env: Env, msg: Bytes, sig: Bytes) {
        let pubkey = env.storage().temporary().get(&"pubkey");
        let _ok = env.crypto().ed25519_verify(&pubkey, &msg, &sig);
    }
}
