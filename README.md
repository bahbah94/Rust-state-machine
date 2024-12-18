# Rust-state-machine
# Rust State Machine (Substrate-style)

A minimal blockchain state machine implementation following Substrate design patterns, demonstrating core concepts of blockchain architecture.

## Core Components

### Pallets
Self-contained modules that manage specific state and logic:
- `System`: Manages block numbers and nonces
- `Balances`: Handles token transfers and account balances
- `Proof of Existence`: Implements document claims and ownership

### Runtime
The main state machine that:
- Combines all pallets
- Processes blocks and extrinsics
- Handles dispatch and execution

### Types
Custom type definitions for:
- AccountId
- Balance
- BlockNumber
- Nonce
- Extrinsics
- Headers

## Architecture
- Uses generic traits for flexible implementation
- Config traits for pallet configuration
- BTreeMap for state storage
- Result types for error handling

## Usage
```rust
let mut runtime = Runtime::new();
// Create and execute blocks with extrinsics
// Handle balance transfers
// Manage proof of existence claims
