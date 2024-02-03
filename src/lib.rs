//! Implements Bitsave protocol for Arbitrum Stylus, providing a Solidity ABI-equivalent


// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Entrypoint is a user data type
//
sol_storage! {
    #[entrypoint]
    pub struct Bitsave {
        uint256 number;
    }
//! Rust implementation of the Counter contract example provided by Foundry.
//! Warning: this code is a template only and has not been audited.
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Bitsave {
    /// Gets the number from storage.
    pub fn number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.number.set(new_number);
        Ok(())
    }

    /// Increments number and updates it values in storage.
    pub fn increment(&mut self) -> Result<(), Vec<u8>> {
        let number = self.number.get();
        self.set_number(number + U256::from(1))
    }
}
