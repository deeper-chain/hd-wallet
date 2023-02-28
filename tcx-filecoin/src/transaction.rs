use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct UnsignedMessage {
    pub to: std::string::String,

    pub from: std::string::String,

    pub nonce: u64,

    pub value: std::string::String,

    pub gas_limit: i64,

    pub gas_fee_cap: std::string::String,

    pub gas_premium: std::string::String,

    pub method: u64,

    pub params: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Signature {
    pub r#type: u32,

    pub data: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SignedMessage {
    pub cid: std::string::String,

    pub message: ::std::option::Option<UnsignedMessage>,

    pub signature: ::std::option::Option<Signature>,
}
