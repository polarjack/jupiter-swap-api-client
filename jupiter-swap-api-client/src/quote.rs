//! Quote data structures for requesting a swap price and handling the response.
//! This is typically used by a DeFi routing or aggregation service on Solana.

use std::str::FromStr;

use crate::route_plan_with_metadata::RoutePlanWithMetadata;
use crate::serde_helpers::field_as_string;
use anyhow::{anyhow, Error};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

// --- Utility Type ---

/// Comma-delimited list of Decentralized Exchange (DEX) labels (e.g., "Raydium,Orca").
type Dexes = String;

// --- Swap Mode Enumeration ---

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
/// Defines the direction of the swap, based on which amount is fixed.
pub enum SwapMode {
    /// The input amount is fixed; slippage occurs on the output amount. (Default)
    #[default]
    ExactIn,
    /// The output amount is fixed (e.g., for payments); slippage occurs on the input amount.
    ExactOut,
}

impl FromStr for SwapMode {
    type Err = Error;

    /// Attempts to convert a string slice into a SwapMode enum.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ExactIn" => Ok(Self::ExactIn),
            "ExactOut" => Ok(Self::ExactOut),
            _ => Err(anyhow!(
                "'{}' is not a valid SwapMode. Expected 'ExactIn' or 'ExactOut'.",
                s
            )),
        }
    }
}

// --- Instruction Version Enumeration ---

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
/// Specifies the swap program instruction version to use.
pub enum InstructionVersion {
    /// Version 1 of the swap instruction format. (Default)
    #[default]
    V1,
    /// Version 2 of the swap instruction format.
    V2,
}

// --- Main Request Structures ---

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Full request payload sent by the client to obtain a swap quote and route plan.
pub struct QuoteRequest {
    /// The mint of the token being swapped (given).
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The mint of the token to be received (wanted).
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount of the input or output token (depending on `swap_mode`), factoring in token decimals.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// The maximum allowed price slippage, measured in basis points (e.g., 50 for 0.5%).
    pub slippage_bps: u16,
    /// The swap direction (ExactIn or ExactOut). Defaults to ExactIn.
    pub swap_mode: Option<SwapMode>,
    /// A comma-separated list of DEXes to explicitly include in the search.
    pub dexes: Option<Dexes>,
    /// A comma-separated list of DEXes to explicitly exclude from the search.
    pub excluded_dexes: Option<Dexes>,
    /// Restricts intermediate tokens to a list known to have stable liquidity.
    pub restrict_intermediate_tokens: Option<bool>,
    /// If true, restricts routing to only direct token pair swaps (no multi-hop).
    pub only_direct_routes: Option<bool>,
    /// If true, the resulting transaction will attempt to fit into a legacy (non-versioned) transaction format.
    pub as_legacy_transaction: Option<bool>,
    /// Optional platform fee to be collected (in basis points).
    pub platform_fee_bps: Option<u16>,
    /// Estimates and restricts the route to fit within a max number of accounts involved. Use with caution.
    pub max_accounts: Option<usize>,
    /// Specifies the swap program instruction version to use (V1 or V2). Defaults to V1.
    pub instruction_version: Option<InstructionVersion>,
}

// Implement Default manually to provide a safer default slippage_bps.
impl Default for QuoteRequest {
    fn default() -> Self {
        QuoteRequest {
            input_mint: Pubkey::default(),
            output_mint: Pubkey::default(),
            amount: 0,
            // Recommended default slippage for safe operation (0.5% or 50 BPS).
            slippage_bps: 50,
            swap_mode: None,
            dexes: None,
            excluded_dexes: None,
            restrict_intermediate_tokens: None,
            only_direct_routes: None,
            as_legacy_transaction: None,
            platform_fee_bps: None,
            max_accounts: None,
            instruction_version: None,
        }
    }
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Internal structure used by the routing engine, excluding fields unnecessary for the core logic.
/// This structure is derived from `QuoteRequest` but omits external/extra configuration fields.
pub struct InternalQuoteRequest {
    /// The mint of the token being swapped (given).
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The mint of the token to be received (wanted).
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount to swap, factoring in the token decimals.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// Allowed slippage in basis points.
    pub slippage_bps: u16,
    /// The swap direction (ExactIn or ExactOut).
    pub swap_mode: Option<SwapMode>,
    /// DEXes explicitly included in the search.
    pub dexes: Option<Dexes>,
    /// DEXes explicitly excluded from the search.
    pub excluded_dexes: Option<Dexes>,
    /// Restricts intermediate tokens to a safe, liquid set.
    pub restrict_intermediate_tokens: Option<bool>,
    /// If true, only direct token routes are considered.
    pub only_direct_routes: Option<bool>,
    /// If true, attempts to fit the quote into a legacy transaction.
    pub as_legacy_transaction: Option<bool>,
    /// Platform fee in basis points.
    pub platform_fee_bps: Option<u16>,
    /// Maximum estimated number of accounts involved in the route.
    pub max_accounts: Option<usize>,
    /// Specifies the swap program instruction version to use (V1 or V2).
    pub instruction_version: Option<InstructionVersion>,
}

impl From<QuoteRequest> for InternalQuoteRequest {
    /// Converts a client's QuoteRequest into the simplified InternalQuoteRequest used for core routing.
    fn from(request: QuoteRequest) -> Self {
        InternalQuoteRequest {
            input_mint: request.input_mint,
            output_mint: request.output_mint,
            amount: request.amount,
            slippage_bps: request.slippage_bps,
            swap_mode: request.swap_mode,
            dexes: request.dexes,
            excluded_dexes: request.excluded_dexes,
            restrict_intermediate_tokens: request.restrict_intermediate_tokens,
            only_direct_routes: request.only_direct_routes,
            as_legacy_transaction: request.as_legacy_transaction,
            platform_fee_bps: request.platform_fee_bps,
            max_accounts: request.max_accounts,
            instruction_version: request.instruction_version,
        }
    }
}

// --- Response Sub-Structure ---

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Details about the platform fee collected for the swap.
pub struct PlatformFee {
    /// The fee amount collected (factoring in token decimals).
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// The fee percentage collected, in basis points (BPS).
    pub fee_bps: u16,
}

// --- Main Response Structure ---

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// The final response containing the best quote and the path to execute the swap.
pub struct QuoteResponse {
    /// The mint of the token provided by the user.
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The final input amount needed for the route (may differ slightly if SwapMode::ExactOut).
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    /// The mint of the token to be received by the user.
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The final output amount expected from the route (may differ slightly if SwapMode::ExactIn).
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    /// The threshold amount on the non-fixed side of the swap. Used for validation/slippage.
    /// (e.g., minimum out for ExactIn, maximum in for ExactOut).
    #[serde(with = "field_as_string")]
    pub other_amount_threshold: u64,
    /// The mode used for calculating the quote (ExactIn or ExactOut).
    pub swap_mode: SwapMode,
    /// The slippage basis points used for the quote calculation.
    pub slippage_bps: u16,
    /// Details on the platform fee collected, if any.
    pub platform_fee: Option<PlatformFee>,
    /// The percentage impact the swap will have on the liquidity pool price.
    pub price_impact_pct: Decimal,
    /// The detailed list of steps (swaps) that make up the final route.
    pub route_plan: RoutePlanWithMetadata,
    /// The slot number of the Solana network at the time the quote was generated. (Default 0)
    #[serde(default)]
    pub context_slot: u64,
    /// The time taken (in seconds) to generate this quote. (Default 0.0)
    #[serde(default)]
    pub time_taken: f64,
}
