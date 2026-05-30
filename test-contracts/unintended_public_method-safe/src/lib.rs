use soroban_sdk::{contractimpl, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn main(env: Env) {
        // do something
    }
    
    pub fn transfer(env: Env) {
        // intended entrypoint
    }
}
