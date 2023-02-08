use bytes::BytesMut;
use prost::Message;
use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
//use tcx::api::InitTokenCoreXParam;
use tcx::api::{
    keystore_common_derive_param::Derivation, AccountsResponse, HdStoreImportParam,
    KeystoreCommonAccountsParam, KeystoreCommonDeriveParam, KeystoreCommonExportResult,
    PublicKeyParam, PublicKeyResult, SignParam, WalletKeyParam, WalletResult,
};
use tcx::{call_api, get_last_err_message};
//use tcx_constants;
use tcx_bch;
use tcx_primitive;
use tcx_substrate::{self, SubstrateRawTxIn, SubstrateTxOut};
// use tcx_tron;
// use tcx_tezos;
// use tcx_filecoin;

pub fn encode_message(msg: impl Message) -> Vec<u8> {
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    buf.to_vec()
}

fn _to_c_char(str: &str) -> *const c_char {
    CString::new(str).unwrap().into_raw()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "generate" => println!("mnemonic: \"{}\"", tcx_primitive::generate_mnemonic()),

        "substrate_pk_from_seed" => println!(
            "public key from seed: {}",
            tcx_primitive::recover_pk_from_seed(&args[2])
        ),
        "substrate_pk_from_mnemonic" => println!(
            "public key from mnemonic: {}",
            tcx_primitive::recover_pk_from_mnemonic(&args[2])
        ),
        "substrate_sk_drive_from_mnemonic" => println!(
            "private key from mnemonic: {}",
            tcx_primitive::recover_sk_drive_from_mnemoic(&args[2], &args[3])
        ),
        "substrate_ss58" => println!(
            "ss58 address from mnemonic: {}",
            tcx_primitive::recover_address_from_mnemonic(&args[2])
        ),
        "substrate_pk_drive_from_mnemonic" => println!(
            "public key from mnemonic: {}",
            tcx_primitive::recover_pk_drive_from_mnemoic(&args[2], &args[3])
        ),

        "export_from_secret_key" => println!(
            "export_from_secret_key: {:?}",
            tcx_substrate::export_from_secret_key()
        ),
        //————————Substrate———————————
        "bip39_mnemonic_convert_seed" => println!(
            "bip39 mnemonic convert seed: {:?} ",
            tcx_primitive::mnemonic_convert_seed(&args[2])
        ),
        "bch_address_from_pub_key" => println!(
            "bch_address from pub key: {}",
            tcx_bch::bch_address_from_pub_key(&args[2])
        ),
        "bch_address_from_pri_key" => println!(
            "bch_address from pri key: {}",
            tcx_bch::bch_address_from_pri_key(&args[2])
        ),
        "bch_sign_tx" => println!(
            "bch sign transaction: {}",
            tcx_bch::bch_sign_to_tx(&args[2], args[3].parse::<i64>().unwrap(), &args[4])
        ),
        "bch_account_create" => {
            println!("keystore info: {}", tcx_bch::bch_account_create(&args[2]))
        }
        "bch_account_recover" => println!(
            "keystore info: {}",
            tcx_bch::bch_account_recover(&args[2], &args[3])
        ),

        _ => main2(&args[1], &args[2]),
    }
}

fn main2(command: &str, content: &str) {
    match command {
        // api for substrate
        "api_substrate_pk_from_mnemonic" => hd_import(content),
        "get_public" => get_public(),

        _ => println!("error"),
    }
}
fn hd_import(mnemonic: &str) {
    let import_param = HdStoreImportParam {
        mnemonic: mnemonic.to_string(),
        password: "".to_string(),
        source: "MNEMONIC".to_string(),
        name: "call_tcx_api".to_string(),
        password_hint: "deeper".to_string(),
        overwrite: true,
    };
    let ret_bytes = call_api("hd_store_import", import_param).unwrap();
    let ret: WalletResult = WalletResult::decode(ret_bytes.as_slice()).unwrap();
    println!("ret {:?}", ret);
}

fn get_public() {
    let empty = WalletKeyParam {
        id: "".to_string(),
        password: "".to_string(),
    };
    let _ = call_api("scan_keystores", empty);

    let param = WalletKeyParam {
        id: "ef403180-9a9e-4882-a99e-441ad9da7645".to_string(),
        password: "".to_string(),
    };
    let ret = call_api("export_mnemonic", param).unwrap();
    let result: KeystoreCommonExportResult =
        KeystoreCommonExportResult::decode(ret.as_slice()).unwrap();

    println!("KeystoreCommonExportResult {:?}", result);

    let derivations = vec![Derivation {
        chain_type: "KUSAMA".to_string(),
        path: "".to_string(),
        network: "".to_string(),
        seg_wit: "".to_string(),
        chain_id: "".to_string(),
        curve: "".to_string(),
    }];
    let param = KeystoreCommonDeriveParam {
        id: "ef403180-9a9e-4882-a99e-441ad9da7645".to_string(),
        password: "".to_string(),
        derivations,
    };

    let derived_accounts_bytes = call_api("keystore_common_derive", param).unwrap();
    let derived_accounts: AccountsResponse =
        AccountsResponse::decode(derived_accounts_bytes.as_slice()).unwrap();
    println!("derived_accounts {:?}", derived_accounts);

    let unsigned_msg = "0x0123456789";
    let input = SubstrateRawTxIn {
        raw_data: unsigned_msg.to_string(),
    };

    let input_value = encode_message(input);
    let tx = SignParam {
        id: "ef403180-9a9e-4882-a99e-441ad9da7645".to_string(),
        key: Some(tcx::api::sign_param::Key::Password("".to_string())),
        chain_type: "KUSAMA".to_string(),
        address: derived_accounts.accounts.first().unwrap().address.clone(),
        input: Some(::prost_types::Any {
            type_url: "imtoken".to_string(),
            value: input_value.clone(),
        }),
    };

    let ret = call_api("sign_tx", tx).unwrap();
    let output: SubstrateTxOut = SubstrateTxOut::decode(ret.as_slice()).unwrap();

    println!("SubstrateTxOut {:?}", output);
}
