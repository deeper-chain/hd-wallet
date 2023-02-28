use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SubstrateKeystoreParam {
    pub keystore: std::string::String,

    pub password: std::string::String,

    pub chain_type: std::string::String,

    pub overwrite: bool,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExportSubstrateKeystoreResult {
    pub keystore: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SubstrateRawTxIn {
    pub raw_data: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SubstrateTxOut {
    pub signature: std::string::String,
}
