//! Implements Bitsave protocol for Arbitrum Stylus, providing a Solidity ABI-equivalent


// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]

mod interfaces;
mod childBitsave;

extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use std::ops::Deref;
use alloy_primitives::{Address, StorageKey};
/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};
use stylus_sdk::{deploy::RawDeploy, call::Call};
use stylus_sdk::storage::{StorageAddress, StorageB256, StorageB8, StorageMap, StorageString, StorageU256, StorageUint, StorageVec};
use crate::childBitsave::ChildBitsave;

// Entrypoint is a user data type
// sol_storage! {
//     #[entrypoint]
//     pub struct Bitsave {
//         uint256 number;
//     }
// }

sol_storage! {
    #[entrypoint]
    pub struct Bitsave {
        uint256 number;
        address usersAddresses;
        mapping(address => address) addressToUserBs;
        // one source of truth structure
       mapping(address => ChildBitsave) addressToChildBitsave;
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Bitsave {

    #[payable]
    pub fn join_bitsave(&mut self, input: Vec<u8>) -> Result<Address, Vec<u8>> {
        let endownment = U256::from_be_bytes::<32>(
            input[1..].try_into().unwrap()
        );
        // todo: fix this to ChildBitsave
        let child_address: Address = unsafe {
            RawDeploy::new()
                .flush_storage_cache()
                .deploy(
                    &input[..],
                    endownment
                )?
        };

        // todo: map the child contract to the user address
        // self.addressToChildBitsave
        self.addressToUserBs.insert(msg::sender(), child_address);
        Ok(child_address)
    }

    pub fn get_user_child_contract(&self) -> Result<Address, Vec<u8>> {
        Ok(self.addressToUserBs.get(msg::sender()))
    }

    pub fn create_savings(&mut self) -> Result<bool, Vec<u8>> {
        Ok(true)
    }

    pub fn increment_savings(&mut self) -> Result<bool, Vec<u8>> {
        Ok(true)
    }

    pub fn withdraw_savings(&mut self) -> Result<bool, Vec<u8>> {
        Ok(true)
    }
}
