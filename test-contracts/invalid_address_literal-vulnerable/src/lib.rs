use soroban_sdk::{contractimpl, Env, Address};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn transfer_to_admin(env: Env) {
        let admin = Address::from_str(&env, "invalid_g_address");
    }
}
