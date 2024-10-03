# Token Contract with Freeze/Unfreeze Functionality

This Rust project is a Soroban smart contract designed to manage token transfers, with additional functionality to freeze and unfreeze specific accounts. The contract consists of two main files: `contract.rs` and `main.rs`.

## Table of Contents

- [Project Overview](#project-overview)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Building and Running](#building-and-running)
- [Contract Details](#contract-details)
  - [Freeze and Unfreeze Functions](#freeze-and-unfreeze-functions)
  - [Transfer Function](#transfer-function)
- [Contract Deployment](#contract-deployment)
- [Testing](#testing)
- [License](#license)

## Project Overview

The main purpose of this project is to implement a token contract that supports freezing and unfreezing of accounts. This is useful for regulatory compliance, preventing malicious activity, or temporarily disabling the transfer capability of specific accounts.

The `TokenContract` struct implements three main functions:
1. `freeze_account`: Freezes a specified account.
2. `unfreeze_account`: Unfreezes a specified account.
3. `transfer`: Performs a token transfer from one account to another, but only if the sender's account is not frozen.

## Getting Started

### Prerequisites

Ensure that you have the following dependencies installed:

- [Rust](https://www.rust-lang.org/): The Rust programming language.
- [Soroban SDK](https://soroban.stellar.org/sdk): The Soroban SDK for building smart contracts on the Stellar blockchain.

You can install Rust by running:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


Contract Address

GAXUAI63OZVWOROVK5Y2RKQLAPJTFN3RK4H3LWB2WNZQG3TC73JLREI2
