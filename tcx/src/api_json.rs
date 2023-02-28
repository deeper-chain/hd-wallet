use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TcxAction {
    pub method: std::string::String,
    pub param: serde_json::Value,
}

pub enum Operation {
    InitTokenCoreXParam,
    ExportPrivateKeyParam,
}

/// A common response when error occurred.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Response {
    pub is_success: bool,
    pub error: std::string::String,
}

/// Initialization

/// FUNCTION: init_token_core_x(InitTokenCoreXParam)
///
/// initialize tcx by passing keystore folder and xpub encryption params

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct InitTokenCoreXParam {
    pub file_dir: std::string::String,
    pub xpub_common_key: std::string::String,
    pub xpub_common_iv: std::string::String,
    pub is_debug: bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExportPrivateKeyParam {
    pub id: std::string::String,

    pub password: std::string::String,

    pub chain_type: std::string::String,

    pub network: std::string::String,

    pub main_address: std::string::String,

    pub path: std::string::String,
}
///
////// Keystore Common
///
///// FUNCTION: keystore_common_verify(WalletKeyParam) -> Response
/////
///// verify the password of the keystore
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct WalletKeyParam {
    pub id: std::string::String,

    pub password: std::string::String,
}
/// Hd Store

/// FUNCTION: hd_store_create(HdStoreCreateParam): WalletResult
///
/// create a new hd keystore
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct HdStoreCreateParam {
    pub password: std::string::String,

    pub password_hint: std::string::String,

    pub name: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct WalletResult {
    pub id: std::string::String,

    pub name: std::string::String,

    pub source: std::string::String,

    pub accounts: ::std::vec::Vec<AccountResponse>,

    pub created_at: i64,
}
/// FUNCTION: hd_store_import(HdStoreImportParam): WalletResult
///
/// create a new hd keystore by mnemonic
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct HdStoreImportParam {
    pub mnemonic: std::string::String,

    pub password: std::string::String,

    pub source: std::string::String,

    pub name: std::string::String,

    pub password_hint: std::string::String,

    pub overwrite: bool,
}
/// FUNCTION: hd_store_derive(HdStoreDeriveParam): AccountsResponse
///
/// derive new accounts from a hd keystore
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct KeystoreCommonDeriveParam {
    pub id: std::string::String,

    pub password: std::string::String,

    pub derivations: ::std::vec::Vec<keystore_common_derive_param::Derivation>,
}

pub mod keystore_common_derive_param {
    use super::{Deserialize, Serialize};
    #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
    pub struct Derivation {
        pub chain_type: std::string::String,

        pub path: std::string::String,

        pub network: std::string::String,

        pub seg_wit: std::string::String,

        pub chain_id: std::string::String,

        pub curve: std::string::String,
    }
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub chain_type: std::string::String,

    pub address: std::string::String,

    pub path: std::string::String,

    pub extended_xpub_key: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct AccountsResponse {
    pub accounts: ::std::vec::Vec<AccountResponse>,
}
/// FUNCTION: hd_store_export(KeystoreCommonExportResult): KeystoreCommonExistsResult
///
/// export the mnemonic from a hd keystore
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct KeystoreCommonExportResult {
    pub id: std::string::String,
    pub r#type: i32,

    pub value: std::string::String,
}
/// Private Key Store

/// FUNCTION: private_key_store_import(PrivateKeyStoreImportParam): WalletResult
///
/// create a new private key keystore by a private key
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PrivateKeyStoreImportParam {
    pub private_key: std::string::String,

    pub password: std::string::String,

    pub name: std::string::String,

    pub password_hint: std::string::String,

    pub overwrite: bool,

    pub encoding: std::string::String,
}
/// FUNCTION: private_key_store_export(PrivateKeyStoreExportParam): KeystoreCommonExportResult
///
/// export the private key from a private key keystore
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PrivateKeyStoreExportParam {
    pub id: std::string::String,

    pub password: std::string::String,

    pub chain_type: std::string::String,

    pub network: std::string::String,
}
/// Keystore Common

// FUNCTION: keystore_common_delete(WalletKeyParam) -> Response
//
// delete the keystore

/// FUNCTION: keystore_common_exists(KeystoreCommonExistsParam): KeystoreCommonExistsResult
///
/// Check is there a keystore was generate by the special privateKey or mnemonic
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct KeystoreCommonExistsParam {
    pub r#type: i32,

    pub value: std::string::String,

    pub encoding: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct KeystoreCommonExistsResult {
    pub is_exists: bool,

    pub id: std::string::String,
}
/// FUNCTION: keystore_common_accounts(KeystoreCommonAccountsParam): AccountsResponse
///
/// List all accounts from the keystore
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct KeystoreCommonAccountsParam {
    pub id: std::string::String,
}
/// Sign Transaction

/// FUNCTION: sign_tx(SignParam)
///
/// Sign transaction. This api is used for sign any chain_type, you should build the right TxInput instance and
/// put it in the `input` field
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SignParam {
    pub id: std::string::String,

    pub chain_type: std::string::String,

    pub address: std::string::String,

    pub input: ::std::option::Option<Vec<u8>>,

    pub key: ::std::option::Option<sign_param::Key>,
}
pub mod sign_param {
    use super::{Deserialize, Serialize};
    #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
    pub enum Key {
        Password(std::string::String),

        DerivedKey(std::string::String),
    }
}
/// Other
// TODO: annotate following message usage

/// btc-fork
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalAddressParam {
    pub id: std::string::String,

    pub chain_type: std::string::String,

    pub external_idx: u32,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalAddressResult {
    pub address: std::string::String,

    pub derived_path: std::string::String,

    pub r#type: std::string::String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExternalAddressExtra {
    pub enc_xpub: std::string::String,

    pub external_address: ::std::option::Option<external_address_extra::ExternalAddress>,
}
pub mod external_address_extra {
    use super::{Deserialize, Serialize};
    #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
    pub struct ExternalAddress {
        pub address: std::string::String,

        pub derived_path: std::string::String,

        pub r#type: std::string::String,
    }
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BtcForkDeriveExtraParam {
    pub network: std::string::String,

    pub seg_wit: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct HdStoreExtendedPublicKeyParam {
    pub id: std::string::String,

    pub password: std::string::String,

    pub chain_type: std::string::String,

    pub address: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct HdStoreExtendedPublicKeyResponse {
    pub extended_public_key: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PublicKeyParam {
    pub id: std::string::String,

    pub chain_type: std::string::String,

    pub address: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PublicKeyResult {
    pub id: std::string::String,

    pub chain_type: std::string::String,

    pub address: std::string::String,

    pub public_key: std::string::String,
}
/// only support two types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    Mnemonic = 0,
    PrivateKey = 1,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct VerifyDerivedKeyParam {
    pub id: std::string::String,

    pub derived_key: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DerivedKeyResult {
    pub id: std::string::String,

    pub derived_key: std::string::String,
}
/// Only used in Android or iOS

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CacheDerivedKeyResult {
    pub id: std::string::String,

    pub enable_derived_key: bool,

    pub mode: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct WalletId {
    pub id: std::string::String,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BiometricModeResult {
    pub mode: std::string::String,
}
