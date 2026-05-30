#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct CatchUnwindVulnerable;

#[contractimpl]
impl CatchUnwindVulnerable {
    /// Uses catch_unwind directly — should trigger `catch-unwind` (High).
    /// catch_unwind is undefined behavior in WASM targets and will abort the transaction.
    pub fn risky_with_direct(env: Env) -> bool {
        let result = catch_unwind(|| {
            // Some code that might panic
            true
        });
        result.is_ok()
    }

    /// Uses std::panic::catch_unwind — should trigger `catch-unwind` (High).
    pub fn risky_with_std_panic(env: Env) -> bool {
        let result = std::panic::catch_unwind(|| {
            // Some code that might panic
            42
        });
        result.is_ok()
    }

    /// Uses absolute path ::std::panic::catch_unwind — should trigger `catch-unwind` (High).
    pub fn risky_with_absolute_path(env: Env) -> bool {
        let result = ::std::panic::catch_unwind(|| {
            // Some code that might panic
            "hello"
        });
        result.is_ok()
    }

    /// Multiple catch_unwind calls — should trigger multiple findings (High).
    pub fn multiple_catches(env: Env) {
        let _ = catch_unwind(|| {
            // First catch
        });

        let _ = std::panic::catch_unwind(|| {
            // Second catch
        });
    }
}
