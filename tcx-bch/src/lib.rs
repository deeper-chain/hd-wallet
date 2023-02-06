use core::result;

mod address;
mod transaction;

use serde_json::Value;

use tcx_chain::KeystoreGuard;
use tcx_chain::{HdKeystore, Keystore, Metadata};
use tcx_constants::CurveType;
use tcx_constants::{CoinInfo};

const BIP_PATH: &str = "m/44'/145'/0'";

pub type Result<T> = result::Result<T, failure::Error>;

#[macro_use]
extern crate failure;

pub use address::BchAddress;
pub use transaction::BchTransaction;
pub use crate::address::{bch_address_from_pub_key, bch_address_from_pri_key};
pub use crate::transaction::bch_sign_to_tx;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "bch_convert_to_legacy_address_failed# address: {}", _0)]
    ConvertToLegacyAddressFailed(String),
    #[fail(display = "bch_convert_to_cash_address_failed# address: {}", _0)]
    ConvertToCashAddressFailed(String),
    #[fail(display = "construct_bch_address_failed# address: {}", _0)]
    ConstructBchAddressFailed(String),
}

pub fn bch_account_create(password: &str) -> serde_json::Value {
    let mut meta = Metadata::default();
    meta.name = "BCH".to_string();
    meta.password_hint = 5.to_string();

    let mut keystore = Keystore::Hd(HdKeystore::new(password, meta));
    let bch_coin = CoinInfo {
        coin: "BITCOINCASH".to_string(),
        derivation_path: BIP_PATH.to_string(),
        curve: CurveType::SECP256k1,
        network: "MAINNET".to_string(),
        seg_wit: "NONE".to_string(),
    };
    let mut guard = KeystoreGuard::unlock_by_password(&mut keystore, password).unwrap();

    let _ = guard
        .keystore_mut()
        .derive_coin::<BchAddress>(&bch_coin)
        .unwrap();

    let json_str = guard.keystore_mut().to_json();
    let v: Value = serde_json::from_str(&json_str).unwrap();
    v
}

pub fn bch_account_recover(mnemonic: &str, password: &str) -> serde_json::Value {
    let mut meta = Metadata::default();
    meta.name = "BCH".to_string();
    meta.password_hint = 5.to_string();
    
    let mut keystore =
        Keystore::Hd(HdKeystore::from_mnemonic(mnemonic, password, meta).unwrap());

    let bch_coin = CoinInfo {
        coin: "BITCOINCASH".to_string(),
        derivation_path: BIP_PATH.to_string(),
        curve: CurveType::SECP256k1,
        network: "MAINNET".to_string(),
        seg_wit: "NONE".to_string(),
    };

    let mut guard = KeystoreGuard::unlock_by_password(&mut keystore, password).unwrap();

    let _ = guard
        .keystore_mut()
        .derive_coin::<BchAddress>(&bch_coin)
        .unwrap();
    let json_str = guard.keystore_mut().to_json();
    let v: Value = serde_json::from_str(&json_str).unwrap();
    v
}

#[cfg(test)]
mod tests {
    use crate::BchAddress;

    use serde_json::Value;

    use tcx_chain::KeystoreGuard;
    use tcx_chain::{HdKeystore, Keystore, Metadata};
    use tcx_constants::CurveType;
    use tcx_constants::{CoinInfo, TEST_MNEMONIC, TEST_PASSWORD};

    const BIP_PATH: &str = "m/44'/145'/0'";

    #[test]
    fn bch_create() {
        let mut meta = Metadata::default();
        meta.name = "CreateTest".to_string();

        let mut keystore = Keystore::Hd(HdKeystore::new(TEST_PASSWORD, meta));

        let bch_coin = CoinInfo {
            coin: "BITCOINCASH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        };
        let mut guard = KeystoreGuard::unlock_by_password(&mut keystore, TEST_PASSWORD).unwrap();

        let _ = guard
            .keystore_mut()
            .derive_coin::<BchAddress>(&bch_coin)
            .unwrap();

        let json_str = guard.keystore_mut().to_json();
        let v: Value = serde_json::from_str(&json_str).unwrap();

        let active_accounts = v["activeAccounts"].as_array().unwrap();
        assert_eq!(1, active_accounts.len());
        let account = active_accounts.first().unwrap();
        let address = account["address"].as_str().unwrap();
        assert!(!address.is_empty());
        let path = account["derivationPath"].as_str().unwrap();
        assert_eq!(BIP_PATH, path);
        let coin = account["coin"].as_str().unwrap();
        assert_eq!("BITCOINCASH", coin);
    }

    #[test]
    fn bch_recover() {
        let mut meta = Metadata::default();
        meta.name = "RecoverTest".to_string();

        let mut keystore =
            Keystore::Hd(HdKeystore::from_mnemonic(&TEST_MNEMONIC, &TEST_PASSWORD, meta).unwrap());

        let bch_coin = CoinInfo {
            coin: "BITCOINCASH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
            network: "MAINNET".to_string(),
            seg_wit: "NONE".to_string(),
        };

        let mut guard = KeystoreGuard::unlock_by_password(&mut keystore, TEST_PASSWORD).unwrap();

        let _ = guard
            .keystore_mut()
            .derive_coin::<BchAddress>(&bch_coin)
            .unwrap();
        let json_str = guard.keystore_mut().to_json();
        let v: Value = serde_json::from_str(&json_str).unwrap();

        let active_accounts = v["activeAccounts"].as_array().unwrap();
        assert_eq!(1, active_accounts.len());
        let account = active_accounts.first().unwrap();
        let address = account["address"].as_str().unwrap();

        assert_eq!("qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885", address);

        let path = account["derivationPath"].as_str().unwrap();
        assert_eq!(BIP_PATH, path);
        let coin = account["coin"].as_str().unwrap();
        assert_eq!("BITCOINCASH", coin);
    }
}
