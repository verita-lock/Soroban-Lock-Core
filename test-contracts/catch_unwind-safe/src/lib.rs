#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct CatchUnwindSafe;

#[derive(Debug)]
pub enum Error {
    OperationFailed,
    InvalidInput,
}

#[contractimpl]
impl CatchUnwindSafe {
    /// Uses Result return type instead of catch_unwind — safe approach.
    pub fn safe_operation(env: Env) -> Result<bool, Error> {
        // Perform operation that returns Result
        validate_input()?;
        Ok(true)
    }

    /// Explicitly handles errors with Result instead of relying on panic handling.
    pub fn safe_with_result(env: Env) -> Result<u32, Error> {
        // Use Result for error handling instead of catch_unwind
        let value = compute_safely()?;
        Ok(value)
    }

    /// Uses conditional logic instead of relying on panic safety — safe approach.
    pub fn safe_conditional(env: Env) -> Result<(), Error> {
        if !is_valid() {
            return Err(Error::InvalidInput);
        }
        Ok(())
    }

    /// Properly propagates errors using ? operator instead of catching panics.
    pub fn safe_with_propagation(env: Env) -> Result<String, Error> {
        let result = perform_operation()?;
        Ok(result)
    }
}

fn validate_input() -> Result<(), Error> {
    Ok(())
}

fn compute_safely() -> Result<u32, Error> {
    Ok(42)
}

fn is_valid() -> bool {
    true
}

fn perform_operation() -> Result<String, Error> {
    Ok("success".to_string())
}
