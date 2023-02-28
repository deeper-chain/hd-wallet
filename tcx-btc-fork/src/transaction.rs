use serde::{Deserialize, Serialize};

/// This file only contains btc chain(including forks) related messages.
// ref:
// - https://en.bitcoin.it/wiki/Transaction
// - https://www.bitcoincash.org/spec/transaction.html

/// FUNCTION: sign_tx(SignParam{input: BtcForkTxInput}): BtcForkSignedTxOutput
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Utxo {
    pub tx_hash: std::string::String,

    pub vout: i32,

    pub amount: i64,

    pub address: std::string::String,

    pub script_pub_key: std::string::String,

    pub derived_path: std::string::String,

    pub sequence: i64,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BtcForkTxInput {
    pub to: std::string::String,

    pub amount: i64,

    pub unspents: ::std::vec::Vec<Utxo>,

    pub fee: i64,

    pub change_address_index: u32,

    pub change_address: std::string::String,

    pub network: std::string::String,

    pub seg_wit: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BtcForkSignedTxOutput {
    pub signature: std::string::String,

    pub tx_hash: std::string::String,
}
