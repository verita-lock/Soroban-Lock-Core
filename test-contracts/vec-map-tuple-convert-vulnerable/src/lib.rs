#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, Symbol, Val};

#[contract]
pub struct VecMapTupleConvertVulnerable;

#[contractimpl]
impl VecMapTupleConvertVulnerable {
    /// ❌ Converts Map to Vec<(Symbol, Val)> unnecessarily — wastes compute.
    pub fn process(env: Env, map: Map<Symbol, Val>) -> u32 {
        let v = map.to_vec();
        v.len()
    }
}
