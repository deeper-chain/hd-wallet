mod address;
mod signer;
mod transaction;

use thiserror::Error;

pub use address::SolanaAddress;
pub use transaction::{SolanaRawTxIn, SolanaTxOut};

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("invalid_curve_type")]
    InvalidCurveType,

    #[error("cannot_found_account")]
    CannotFoundAccount,

    #[error("invalid_address")]
    InvalidAddress,

    #[error("invalid_format")]
    InvalidFormat,

    #[error("invalid_param")]
    InvalidParam,

    #[error("invalid_number")]
    InvalidNumber,
}
