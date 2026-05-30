#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, Symbol};

#[contract]
pub struct VulnerableContract;

#[contractimpl]
impl VulnerableContract {
    /// Vulnerable: sha256(address + storage-read secret) — both inputs may be known.
    pub fn commit(env: Env, user: Address) {
        let key = Symbol::new(&env, "secret");
        let secret: Bytes = env.storage().instance().get(&key).unwrap();
        // ❌ Both `user` (Address param) and `secret` (storage read) are known to attacker
        let hash = env.crypto().sha256(&(user, secret));
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "commit"), &hash);
    }
}
