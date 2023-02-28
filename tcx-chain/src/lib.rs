//! TokenCore Chain
//! This is an abstract package to define basic chain data structures.
#[cfg_attr(tarpaulin, skip)]
#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! tcx_ensure {
        ($cond:expr, $e:expr) => {
            if !($cond) {
                return Err($e.into());
            }
        };
    }
}

pub use anyhow::Result;

extern crate regex;

mod keystore;
mod signer;

pub use keystore::{
    key_hash_from_mnemonic, key_hash_from_private_key, Account, Address, HdKeystore, Keystore,
    KeystoreGuard, Metadata, PrivateKeystore, Source,
};

pub use signer::{ChainSigner, MessageSigner, TransactionSigner};
