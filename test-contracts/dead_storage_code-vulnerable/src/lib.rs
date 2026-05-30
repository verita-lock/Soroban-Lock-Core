use soroban_sdk::{contractimpl, Env, Address};

pub struct Contract;

// Dead function with storage operations
fn helper(env: Env) {
    env.storage().persistent().set(&"key", &123);
}

#[contractimpl]
impl Contract {
    pub fn main(env: Env) {
        // doesn't call helper
    }
}
