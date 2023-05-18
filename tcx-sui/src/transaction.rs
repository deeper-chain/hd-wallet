use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SuiRawTxIn {
    pub raw_data: std::string::String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SuiTxOut {
    pub signature: std::string::String,
}
