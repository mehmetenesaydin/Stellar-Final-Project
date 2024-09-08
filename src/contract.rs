use crate::contract;
use soroban_sdk::{contractimpl, Env, Address, Symbol};

// Define the Frozen Accounts storage key
const FROZEN_ACCOUNTS: Symbol = Symbol::short("frozen_acc");

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    // Function to freeze an account
    pub fn freeze_account(env: Env, account: Address) {
        let key = (FROZEN_ACCOUNTS, account.clone());
        env.storage().set(&key, &true, StorageType::Temporary); // Mark account as frozen
    }

    // Function to unfreeze an account
    pub fn unfreeze_account(env: Env, account: Address) {
        let key = (FROZEN_ACCOUNTS, account.clone());
        env.storage().set(&key, &false, StorageType::Temporary); // Mark account as unfrozen
    }

    // Updated transfer function to reject transfers if an account is frozen
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // Check if sender account is frozen
        let key = (FROZEN_ACCOUNTS, from.clone());
        if let Some(is_frozen) = env.storage().get::<bool>(&key, StorageType::Temporary) {
            if is_frozen {
                panic!("Account is frozen and cannot transfer tokens");
            }
        }

        // Proceed with the transfer if account is not frozen
        // Transfer logic here (e.g., check balances, update state)
    }
}

fn main() {
    // main.rs can be used for tests or calling the contract
}
