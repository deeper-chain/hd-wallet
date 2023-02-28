use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct EthereumTxIn {
    pub nonce: std::string::String,

    pub to: std::string::String,

    pub value: std::string::String,

    pub gas_price: std::string::String,

    pub gas: std::string::String,

    pub data: std::string::String,

    pub network: std::string::String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct EthereumTxOut {
    pub signature: std::string::String,
}
