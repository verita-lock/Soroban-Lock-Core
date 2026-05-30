#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, Symbol, Val};

#[contract]
pub struct VecMapTupleConvertSafe;

#[contractimpl]
impl VecMapTupleConvertSafe {
    /// ✅ Uses direct Map operations instead of converting to Vec.
    pub fn process(env: Env, map: Map<Symbol, Val>, key: Symbol) -> Option<Val> {
        map.get(key)
    }
}
