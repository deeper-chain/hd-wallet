use super::Error;
use tcx_chain::{Address, Result};
use tcx_constants::CoinInfo;
use tcx_primitive::{PublicKey, ToHex, TypedPublicKey};

pub struct SolanaAddress();

impl Address for SolanaAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        match public_key {
            TypedPublicKey::Ed25519(pk) => {
                let pk_bytes = pk.to_bytes();
                Ok(bs58::encode(pk_bytes).into_string())
            }

            _ => {
                return Err(Error::InvalidCurveType.into());
            }
        }
    }

    fn is_valid(_address: &str, _coin: &CoinInfo) -> bool {
        true
    }
}
