/// This file only contains tron related messages.
// ref: https://developers.tron.network/docs/transaction
use serde::{Deserialize, Serialize};

/// FUNCTION: sign_tx(SignParam{input: TronTxInput}): TronTxOutput
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TronTxInput {
    /// hex string
    pub raw_data: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TronTxOutput {
    /// hex string
    pub signatures: ::std::vec::Vec<std::string::String>,
}
/// FUNCTION: tron_sign_message(SignParam): TronMessageOutput
///
/// This api use the a common struct named `SignParam`, you should
/// build the `TronMessageInput` and put it in the `input` field
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TronMessageInput {
    pub value: std::string::String,

    pub is_hex: bool,

    pub is_tron_header: bool,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TronMessageOutput {
    pub signature: std::string::String,
}
