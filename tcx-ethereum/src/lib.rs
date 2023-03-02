mod address;
mod chain_id;
mod signer;
mod transaction;

pub use crate::address::EthereumAddress;
pub use crate::chain_id::{chain_id_from_network, ChainInfo};
pub use crate::transaction::{EthereumTxIn, EthereumTxOut};
use thiserror::Error;

#[macro_use]
extern crate lazy_static;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("cannot_found_account")]
    CannotFoundAccount,

    #[error("cannot_get_private_key")]
    CannotGetPrivateKey,

    #[error("invalid_nonce")]
    InvalidNonce,

    #[error("invalid_to")]
    InvalidTo,

    #[error("invalid_value")]
    InvalidValue,

    #[error("invalid_gas_price")]
    InvalidGasPrice,

    #[error("invalid_gas")]
    InvalidGas,

    #[error("invalid_data")]
    InvalidData,

    #[error("private_key_unconvert")]
    PrivateKeyUnConvert,

    #[error("invalid_chain_id")]
    InvalidChainId,
}
