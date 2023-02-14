use bitcoin::util::amount;
use prost::Message;
use tcx::api::{
    keystore_common_derive_param::Derivation, AccountsResponse, ExportPrivateKeyParam,
    HdStoreCreateParam, HdStoreImportParam, InitTokenCoreXParam, KeystoreCommonAccountsParam,
    KeystoreCommonDeriveParam, KeystoreCommonExistsParam, KeystoreCommonExistsResult,
    KeystoreCommonExportResult, PrivateKeyStoreImportParam, PublicKeyParam, PublicKeyResult,
    Response, SignParam, WalletKeyParam, WalletResult,
};
use tcx::{call_api, error_handling::Result, get_last_err_message, handler};
use tcx_substrate::{self, SubstrateRawTxIn, SubstrateTxOut};

use tcx_tezos::address::TezosAddress;
use tcx_tezos::transaction::{TezosRawTxIn, TezosTxOut};
use tcx_tron::transaction::{TronMessageInput, TronTxInput, TronTxOutput};

pub fn func(args: Vec<String>) {
    match args[1].as_str() {
        "init_token_core_x" => init_token_core_x(args[2].clone()),
        "scan_keystores" => scan_keystores(),
        "hd_store_create" => hd_store_create(args[2].clone(), args[3].clone()),
        "hd_store_import" => {
            scan_keystores();
            hd_store_import(args[2].clone(), args[3].clone(), args[4].clone());
        }
        "export_mnemonic" => {
            scan_keystores();
            export_mnemonic(args[2].clone(), args[3].clone());
        }
        "keystore_common_derive" => {
            scan_keystores();
            keystore_common_derive(
                args[2].clone(),
                args[3].clone(),
                args[4].clone(),
                args[5].clone(),
                args[6].clone(),
            );
        }
        "private_key_store_import" => {
            private_key_store_import(args[2].clone(), args[3].clone(), args[4].clone())
        }
        "export_private_key" => {
            scan_keystores();
            export_private_key(
                args[2].clone(),
                args[3].clone(),
                args[4].clone(),
                args[5].clone(),
                args[6].clone(),
            );
        }
        "keystore_common_verify" => {
            scan_keystores();
            keystore_common_verify(args[2].clone(), args[3].clone())
        }
        "keystore_common_exists" => {
            keystore_common_exists(args[2].clone(), args[3].clone(), args[4].clone())
        }
        "keystore_common_accounts" => {
            scan_keystores();
            keystore_common_accounts(args[2].clone());
        }
        "get_public_key" => {
            scan_keystores();
            get_public_key(args[2].clone(), args[3].clone());
        }
        "sign_tx" => {
            scan_keystores();
            sign_tx(
                args[2].clone(),
                args[3].clone(),
                args[4].clone(),
                args[5].clone(),
                args[6].clone(),
            );
        }
        "tx_input" => tx_input(args[2..].to_vec()),

        "tx_output" => tx_output(args[2].clone(), args[3].clone()),
        "common_sign_message" => {
            scan_keystores();
            common_sign_message(&args[2], &args[3], &args[4], &args[5], &args[6], &args[7]);
        }
        _ => {}
    }
}

fn common_sign_message(
    id: &str,
    password: &str,
    symbol: &str,
    address: &str,
    path: &str,
    data: &str,
) {
    let path = if path.is_empty() { None } else { Some(path) };
    let data = hex::decode(data).expect("sign output not correct");
    let sign_res =
        handler::common_sign_message(id, password, symbol, address, path, data.as_slice());
    match sign_res {
        Ok(sign_res) => {
            let ret_str = hex::encode(sign_res);
            println!("common_sign_message result {:?}", ret_str);
        }
        Err(e) => {
            println!("common_sign_message err {:?}", e);
        }
    }
}

fn encode_message(msg: impl Message) -> Result<Vec<u8>> {
    let mut buf = bytes::BytesMut::with_capacity(msg.encoded_len());
    msg.encode(&mut buf)?;
    Ok(buf.to_vec())
}

fn print_simple_input(msg: impl Message) {
    let msg = encode_message(msg);
    if let Ok(msg) = msg {
        let ret = hex::encode(msg);
        println!("input {:?}", ret)
    } else {
        println!("substrate_input not correct");
    }
}

fn tx_output(chain_type: String, output: String) {
    let ret = hex::decode(output).expect("sign output not correct");
    match chain_type.as_str() {
        "BITCOINCASH" | "LITECOIN" => {}
        "TRON" => {
            let output: TronTxOutput = TronTxOutput::decode(ret.as_slice()).unwrap();
            println!("{} output {:?}", chain_type, output.signatures[0]);
        }
        "NERVOS" => {}
        "POLKADOT" | "KUSAMA" | "DEEPER" => {
            let output: SubstrateTxOut = SubstrateTxOut::decode(ret.as_slice()).unwrap();
            println!(
                "{} output {:?}",
                chain_type,
                output.signature[4..].to_string()
            );
        }
        "FILECOIN" => {}
        "TEZOS" => {
            let output: TezosTxOut = TezosTxOut::decode(ret.as_slice()).unwrap();
            println!("{} output {:?}", chain_type, output);
        }
        _ => println!("unsuported chain"),
    }
}

fn tx_input(input: Vec<String>) {
    match input[0].as_str() {
        "BITCOINCASH" | "LITECOIN" => {}
        "TRON" => {
            let tx = TronTxInput {
                raw_data: input[1].clone(),
            };
            print_simple_input(tx);
        }
        "NERVOS" => {}
        "POLKADOT" | "KUSAMA" | "DEEPER" => {
            let tx = SubstrateRawTxIn {
                raw_data: input[1].clone(),
            };
            print_simple_input(tx);
        }
        "FILECOIN" => {}
        "TEZOS" => {
            let tx = TezosRawTxIn {
                raw_data: input[1].clone(),
            };
            print_simple_input(tx);
        }
        _ => println!("unsuported chain"),
    }
}

fn sign_tx(id: String, password: String, chain_type: String, address: String, input: String) {
    let input_value = hex::decode(input).expect("input not correct");

    let param = SignParam {
        id,
        key: Some(tcx::api::sign_param::Key::Password(password.to_string())),
        chain_type,
        address,
        input: Some(::prost_types::Any {
            type_url: "deeper".to_string(),
            value: input_value,
        }),
    };

    let ret = call_api("sign_tx", param).unwrap();
    let ret_str = hex::encode(ret);
    println!("sign_tx result {:?}", ret_str);
}

fn get_public_key(id: String, address: String) {
    let param: PublicKeyParam = PublicKeyParam {
        id,
        chain_type: "TEZOS".to_string(),
        address,
    };
    let ret_bytes = call_api("get_public_key", param).unwrap();
    let public_key_result: PublicKeyResult = PublicKeyResult::decode(ret_bytes.as_slice()).unwrap();
    println!("tron get_public_key {:?}", public_key_result);
}

fn keystore_common_accounts(id: String) {
    let param = KeystoreCommonAccountsParam { id };
    let accounts_ret = call_api("keystore_common_accounts", param).unwrap();
    let ret = AccountsResponse::decode(accounts_ret.as_slice()).unwrap();
    println!("keystore_common_accounts {:?}", ret);
}

// encoding : TEZOS or ""; key_type privatekey or mnemonic
fn keystore_common_exists(key_type: String, value: String, encoding: String) {
    let key_type: i32 = match key_type.as_str() {
        "privatekey" => 1,
        "mnemonic" => 2,
        _ => {
            println!("key type not correct");
            return;
        }
    };

    let param: KeystoreCommonExistsParam = KeystoreCommonExistsParam {
        r#type: key_type,
        value,
        encoding,
    };

    let ret_bytes = call_api("keystore_common_exists", param).unwrap();
    let result: KeystoreCommonExistsResult =
        KeystoreCommonExistsResult::decode(ret_bytes.as_slice()).unwrap();

    println!("keystore_common_exists result {:?}", result);
}

fn keystore_common_delete(id: String, password: String) {
    let param: WalletKeyParam = WalletKeyParam {
        id,
        password: "WRONG PASSWORD".to_string(),
    };

    let ret = call_api("keystore_common_delete", param);
    if let Ok(ret_bytes) = ret {
        let ret: Response = Response::decode(ret_bytes.as_slice()).unwrap();
        println!("keystore_common_verify result {:?}", ret);
    } else {
        println!("keystore_common_verify wrong password");
    }
}

fn keystore_common_verify(id: String, password: String) {
    let param: WalletKeyParam = WalletKeyParam { id, password };

    let ret_bytes = call_api("keystore_common_verify", param).unwrap();
    let result: Response = Response::decode(ret_bytes.as_slice()).unwrap();
    println!("keystore_common_verify resutl {:?}", result);
}

fn export_private_key(
    id: String,
    password: String,
    chain_type: String,
    main_address: String,
    derive_path: String,
) {
    let export_param = ExportPrivateKeyParam {
        id,
        password,
        chain_type,
        network: "".to_string(),
        main_address,
        path: derive_path,
    };

    let export_pk_bytes = call_api("export_private_key", export_param).unwrap();
    let export_pk: KeystoreCommonExportResult =
        KeystoreCommonExportResult::decode(export_pk_bytes.as_slice()).unwrap();
    println!("{:#?}", export_pk);
}

fn private_key_store_import(name: String, private_key: String, password: String) {
    let param: PrivateKeyStoreImportParam = PrivateKeyStoreImportParam {
        private_key,
        password,
        name,
        password_hint: "".to_string(),
        overwrite: true,
        encoding: "".to_string(),
    };

    let ret = call_api("private_key_store_import", param).unwrap();
    let result = WalletResult::decode(ret.as_slice()).unwrap();
    println!("wallet private_key_store_import {:?}", result);
}

fn keystore_common_derive(
    id: String,
    password: String,
    chain_type: String,
    derive_path: String,
    network: String,
) {
    let derivation = Derivation {
        chain_type,
        path: derive_path,
        network,
        seg_wit: "".to_string(),
        chain_id: "".to_string(),
        curve: "".to_string(),
    };
    println!("keystore_common_derive {:?} ", derivation);
    let param = KeystoreCommonDeriveParam {
        id,
        password,
        derivations: vec![derivation],
    };

    let ret = call_api("keystore_common_derive", param).unwrap();
    let result: AccountsResponse = AccountsResponse::decode(ret.as_slice()).unwrap();
    println!("keystore_common_derive {:?}", result);
}

fn export_mnemonic(id: String, password: String) {
    let param = WalletKeyParam { id, password };
    let ret = call_api("export_mnemonic", param).unwrap();
    let result: KeystoreCommonExportResult =
        KeystoreCommonExportResult::decode(ret.as_slice()).unwrap();

    println!("export_mnemonic {:?}", result);
}

fn hd_store_import(name: String, mnemonic: String, password: String) {
    let param = HdStoreImportParam {
        mnemonic,
        password,
        source: "MNEMONIC".to_string(),
        name,
        password_hint: "".to_string(),
        overwrite: true,
    };
    let ret = call_api("hd_store_import", param).unwrap();
    let wallet: WalletResult = WalletResult::decode(ret.as_slice()).unwrap();
    println!("wallet imported {:?}", wallet);
}

fn hd_store_create(name: String, password: String) {
    let param = HdStoreCreateParam {
        password,
        password_hint: "".to_string(),
        name,
    };

    let ret = call_api("hd_store_create", param).unwrap();
    let wallet: WalletResult = WalletResult::decode(ret.as_slice()).unwrap();
    println!("wallet created {:?}", wallet);
}

fn scan_keystores() {
    let empty = WalletKeyParam {
        id: "".to_string(),
        password: "".to_string(),
    };
    let _ = call_api("scan_keystores", empty).expect("scan_keystores error");
}

fn init_token_core_x(file_dir: String) {
    let p = std::path::Path::new(&file_dir);
    if !p.exists() {
        std::fs::create_dir_all(p).expect("shoud create filedir");
    }

    let param = InitTokenCoreXParam {
        file_dir,
        xpub_common_key: "".to_string(),
        xpub_common_iv: "".to_string(),
        is_debug: false,
    };

    let _ = call_api("scan_keystores", param).expect("should init tcx");
}
