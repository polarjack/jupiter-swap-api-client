use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::serde_helpers::field_as_string;

/// Topologically sorted DAG with additional metadata for rendering
pub type RoutePlanWithMetadata = Vec<RoutePlanStep>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanStep {
    pub swap_info: SwapInfo,
    pub percent: u8,
    /// BPS value (may not be present in lite API responses)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bps: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwapInfo {
    #[serde(with = "field_as_string")]
    pub amm_key: Pubkey,
    pub label: String,
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// An estimation of the input amount into the AMM
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    /// An estimation of the output amount into the AMM
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    /// Fee amount (may not be present in lite API responses)
    #[serde(
        with = "crate::serde_helpers::option_field_as_string",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub fee_amount: Option<u64>,
    /// Fee mint (may not be present in lite API responses)
    #[serde(
        with = "crate::serde_helpers::option_field_as_string",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub fee_mint: Option<Pubkey>,
}
