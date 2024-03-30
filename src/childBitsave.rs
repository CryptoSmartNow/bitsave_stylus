
//! Implements Bitsave protocol for Arbitrum Stylus, providing a Solidity ABI-equivalent

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]

extern crate alloc;

use std::ops::Deref;
use alloy_primitives::Address;
/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};
use stylus_sdk::storage::{StorageAddress, StorageBool, StorageU256};


sol_storage! {
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
    
    // #[entrypoint]
    pub struct ChildBitsave {
        // parent bitsave contract
        address bitsaveAddress;
        // the user with this child contract
        address ownerAddress;
        // stable coin used by this contract
        address stableCoin;
        // list of all savings
        DependentStruct savingsMap;
    }
}

impl SavingDataStruct {
    fn new(amount: U256) -> SavingDataStruct {
        SavingDataStruct {
            amount,
            
        }
    }
}

#[external]
impl ChildBitsave {
    
    fn requireAddress(specific_address: Address) {
        assert_eq!(msg::sender(), specific_address)
    }

    pub fn getSaving(&self, nameOfSaving: String) -> Result<SavingDataStruct, Vec<u8>> {
        // check if user is valid
        // check if saving exists
        let savingFound = self
            .savingsMap.savings
            .get(nameOfSaving);
        // return savings
        todo!()
    }
    
    #[payable]
    pub fn create_saving(&self, useSafeMode: bool) -> Result<(), Vec<u8>> {
        // checks txn is coming from parent contract
        Self::requireAddress(*self.bitsaveAddress);
        // checks gas is covered
        // checks amount and other stuffs
        // creates new saving object
        let new_saving = SavingDataStruct {
            interestAccumulated: 0,
            amount: msg::value(),
            isValid: StorageBool(true),
            isSafeMode: StorageBool(useSafeMode),
            maturityTime: StorageU256(0),
            startTime: StorageU256(0),
            penaltyPercentage: StorageU256(0),
            tokenId: StorageAddress()
        };
        // maps saving to name
        // send a result; to decide structure
    }

    pub fn incrementSaving() -> Result<(), Vec<u8>> {
        todo!()
    }

    pub fn withdrawSaving() -> Result<(), Vec<u8>> {
        todo!()
    }
}

