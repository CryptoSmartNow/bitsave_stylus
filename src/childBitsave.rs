
//! Implements Bitsave protocol for Arbitrum Stylus, providing a Solidity ABI-equivalent

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]

extern crate alloc;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, prelude::*};


sol_storage! {
    // #[entrypoint]
    pub struct ChildBitsave {
        // parent bitsave contract
        address bitsaveAddress;
        // the user with this child contract
        address ownerAddress;
        // stable coin used by this contract
        address stableCoin;
    }

    // structure of saving data
    struct SavingDataStruct {
        bool isValid;
        uint256 amount;
        address tokenId;
        uint25 interestAccumulated;
        uint256 startTime;
        uint penaltyPercentage;
        uint256 maturityTime;
        bool isSafeMode;
    }

    // Savings mappings
    pub struct DependentStruct {
        // mapping of name of saving to individual saving
        mapping(string => SavingDataStruct) savings;
    }
}

#[external]
impl ChildBitsave {

    pub fn getSaving(nameOfSaving: String) -> Result<(), Vec<u8>> {
        todo!()
    }
    pub fn createSaving() -> Result<(), Vec<u8>> {
        Ok(())
    }

    pub fn incrementSaving() -> Result<(), Vec<u8>> {
        todo!()
    }

    pub fn withdrawSaving() -> Result<(), Vec<u8>> {
        todo!()
    }
}

