use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TezosRawTxIn {
    pub raw_data: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TezosTxOut {
    pub signature: std::string::String,

    pub edsig: std::string::String,

    pub sbytes: std::string::String,
}
