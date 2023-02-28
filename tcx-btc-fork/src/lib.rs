pub mod address;
pub mod bip143_with_forkid;
pub mod signer;
pub mod transaction;

use core::result;
use serde::{Deserialize, Serialize};

extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;

#[macro_use]
extern crate tcx_chain;

pub use anyhow::Result;

pub use signer::{BitcoinForkSinger, BtcForkSegWitTransaction, BtcForkTransaction};
use thiserror::Error;
pub use transaction::{BtcForkSignedTxOutput, BtcForkTxInput, Utxo};

pub use address::{BtcForkAddress, PubKeyScript, WifDisplay};
pub use signer::ScriptPubKeyComponent;

#[derive(Error, Debug)]
pub enum Error {
    #[error("decrypt_xpub_error")]
    DecryptXPubError,
    #[error("unsupported_chain")]
    UnsupportedChain,
    #[error("missing_network")]
    MissingNetwork,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalAddress {
    pub address: String,
    #[serde(rename = "type")]
    pub addr_type: String,
    pub derived_path: String,
}
