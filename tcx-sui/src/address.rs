use blake2b_rs::Blake2bBuilder;
use tcx_chain::{Address, Result};
use tcx_constants::CoinInfo;
use tcx_primitive::{PublicKey, TypedPublicKey};

use super::Error;

pub fn digest(ingest: &[u8]) -> Vec<u8> {
    //allocate max length byte
    let mut result = [0u8; 32];
    let mut hasher = Blake2bBuilder::new(32).build();
    hasher.update(ingest);
    hasher.finalize(&mut result);
    result[0..32].to_vec()
}
pub struct SuiAddress();

impl Address for SuiAddress {
    fn from_public_key(public_key: &TypedPublicKey, coin: &CoinInfo) -> Result<String> {
        match public_key {
            TypedPublicKey::Secp256k1(pk) => {
                let mut pk = pk.clone();
                pk.0.compressed = false;
                let mut bytes = pk.to_uncompressed();
                bytes.insert(0, 0x01);
                let address = digest(&bytes);
                let addr_hex = hex::encode(address);
                Ok(addr_hex)
            }
            TypedPublicKey::Ed25519(pk) => {
                let mut bytes = pk.to_bytes();
                bytes.insert(0, 0x00);
                let address = digest(&bytes);
                let addr_hex = hex::encode(address);
                Ok(addr_hex)
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
