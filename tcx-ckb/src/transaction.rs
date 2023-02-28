use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct OutPoint {
    pub tx_hash: std::string::String,

    pub index: i32,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Witness {
    pub lock: std::string::String,

    pub input_type: std::string::String,

    pub output_type: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Script {
    pub args: std::string::String,

    pub code_hash: std::string::String,

    pub hash_type: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct CellInput {
    pub previous_output: ::std::option::Option<OutPoint>,

    pub since: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct CachedCell {
    pub capacity: i64,

    pub lock: ::std::option::Option<Script>,

    pub out_point: ::std::option::Option<OutPoint>,

    pub derived_path: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct CkbTxInput {
    pub inputs: ::std::vec::Vec<CellInput>,

    pub witnesses: ::std::vec::Vec<Witness>,

    pub cached_cells: ::std::vec::Vec<CachedCell>,

    pub tx_hash: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct CkbTxOutput {
    pub tx_hash: std::string::String,

    pub witnesses: ::std::vec::Vec<std::string::String>,
}
