use soroban_sdk::{contractimpl, Env, Address, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn call_other(env: Env, contract: Address) {
        env.invoke_contract(&contract, &Symbol::new(&env, "nonexistent_func"), &());
    }
}
