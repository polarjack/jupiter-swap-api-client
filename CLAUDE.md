# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust client library for the Jupiter Swap API, enabling seamless token swaps on the Solana blockchain. The library provides a simple interface to interact with Jupiter's v6 API endpoints for quote retrieval, transaction creation, and swap instruction generation.

## Build and Development Commands

### Building
```bash
# Build the entire workspace
cargo build

# Build with release optimizations
cargo build --release

# Build specific packages
cargo build -p jupiter-swap-api-client
cargo build -p example
```

### Testing
```bash
# Run all tests in the workspace
cargo test

# Run tests for specific package
cargo test -p jupiter-swap-api-client

# Run a specific test
cargo test [test_name]
```

### Running the Example
```bash
# Run the example (demonstrates full API usage)
cargo run -p example

# With custom API base URL
API_BASE_URL=https://custom.api.url cargo run -p example

# With custom RPC URL
SOLANA_RPC_URL=https://custom.rpc.url cargo run -p example
```

### Code Quality
```bash
# Format all code
cargo fmt

# Check code without building
cargo check

# Check specific package
cargo check -p jupiter-swap-api-client
```

## Architecture

### Workspace Structure

This is a Cargo workspace with two members:
- `jupiter-swap-api-client/`: The core library crate
- `example/`: Example implementation demonstrating API usage

### Core API Flow

The library follows Jupiter's API flow:

1. **Quote Request** (`GET /quote`) - Get best route and pricing for a swap
2. **Swap Transaction** (`POST /swap`) - Get a serialized, ready-to-sign transaction
3. **Swap Instructions** (`POST /swap-instructions`) - Get raw instructions for custom transaction building

### Key Modules

#### `lib.rs`
The main entry point defining `JupiterSwapApiClient` with three primary methods:
- `quote()` - Requests a quote from Jupiter API
- `swap()` - Requests a serialized swap transaction
- `swap_instructions()` - Requests decomposed swap instructions

Error handling uses `ClientError` enum with two variants:
- `RequestFailed` - HTTP errors with status and body
- `DeserializationError` - JSON parsing errors

#### `quote.rs`
Defines quote request/response structures:
- `QuoteRequest` - Main request with extensive routing configuration (DEX selection, slippage, platform fees, etc.)
- `InternalQuoteRequest` - Simplified internal representation (drops `quote_args`)
- `QuoteResponse` - Contains route plan, amounts, slippage, and pricing details
- `SwapInfo` - Individual swap step in a multi-hop route
- `SwapMode` - Enum for `ExactIn` (fix input) vs `ExactOut` (fix output)

The separation of `QuoteRequest` and `InternalQuoteRequest` allows extra args to be passed separately via query parameters.

#### `swap.rs`
Defines swap request/response structures:
- `SwapRequest` - Combines user public key, quote response, and transaction config
- `SwapResponse` - Contains base64-encoded serialized transaction plus metadata (block height, compute units, prioritization)
- `SwapInstructionsResponse` - Decomposed instructions (setup, swap, cleanup, compute budget)
- Internal structures with custom deserialization from Jupiter API's format

The module includes custom base64 serialization/deserialization for transaction bytes.

#### `transaction_config.rs`
Extensive configuration for transaction construction:
- SOL wrapping/unwrapping options
- Fee accounts and destination token accounts
- Compute unit pricing (fixed, auto, or priority level-based)
- Prioritization fee configuration (Jito tips or compute budget)
- Dynamic compute unit limits and slippage
- Legacy vs versioned transaction format
- Shared accounts optimization

Notable: `PrioritizationFeeLamports` has complex serialization logic to support multiple configuration modes.

#### `serde_helpers/`
Custom serialization helpers for Solana types:
- `field_as_string` - Serializes `Pubkey` as string (Jupiter API format)
- `option_field_as_string` - Same for `Option<Pubkey>`

#### `route_plan_with_metadata.rs`
Contains routing metadata structures (not extensively documented here but referenced by `QuoteResponse`).

### External Dependencies

Key dependencies:
- `solana-sdk` (v2) - Solana blockchain primitives
- `reqwest` - HTTP client for API calls
- `serde`/`serde_json` - JSON serialization
- `base64` - Transaction encoding
- `rust_decimal` - Precise decimal arithmetic for amounts

## Important Patterns

### Pubkey Serialization
All `Pubkey` fields use custom serialization via `field_as_string` to match Jupiter API's string format rather than byte arrays.

### Quote to Swap Flow
1. Build `QuoteRequest` with desired swap parameters
2. Call `quote()` to get `QuoteResponse`
3. Use `QuoteResponse` in `SwapRequest` with user's public key
4. Call either `swap()` for full transaction or `swap_instructions()` for manual construction

### Environment Variables
- `API_BASE_URL` - Override Jupiter API endpoint (default: `https://quote-api.jup.ag/v6`)
- `SOLANA_RPC_URL` - Override Solana RPC endpoint (for transaction submission in examples)

## Rust Toolchain

The project pins to Rust 1.87.0 via `rust-toolchain.toml`. Always use this version for development.
