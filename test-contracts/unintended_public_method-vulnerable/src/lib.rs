use soroban_sdk::{contractimpl, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn main(env: Env) {
        // do something
    }
    
    // This is an internal helper but marked public
    pub fn _helper(env: Env) {
        // internal logic
    }
}
