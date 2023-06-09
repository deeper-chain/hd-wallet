use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SolanaRawTxIn {
    pub raw_data: std::string::String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SolanaTxOut {
    pub signature: std::string::String,
}
