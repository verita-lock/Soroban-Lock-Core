#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct AddressCmpSafe;

#[contractimpl]
impl AddressCmpSafe {
    /// Uses require_auth for proper authorization.
    pub fn safe_auth(env: Env, caller: Address) {
        env.require_auth(&caller);
        env.storage()
            .instance()
            .set(&symbol_short!("data"), &42u32);
    }
}
