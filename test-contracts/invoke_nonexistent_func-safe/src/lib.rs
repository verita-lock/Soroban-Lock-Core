use soroban_sdk::{contractimpl, Env, Address, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn call_transfer(env: Env, contract: Address) {
        env.invoke_contract(&contract, &Symbol::new(&env, "transfer"), &());
    }
    
    pub fn call_balance_of(env: Env, contract: Address) {
        env.invoke_contract(&contract, &Symbol::new(&env, "balance_of"), &());
    }
}
