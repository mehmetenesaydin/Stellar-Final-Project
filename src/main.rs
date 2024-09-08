#![no_std]

use soroban_sdk::contract;
use crate::contract::TokenContract;

// Link to the contract module
mod contract;

contract! {
    TokenContract
}

