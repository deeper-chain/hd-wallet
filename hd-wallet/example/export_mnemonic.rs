use std::{
    io::{Read, Write},
    net::TcpStream,
};
use tcx::handler::encode_message;
use tcx::{
    api::{
        keystore_common_derive_param::Derivation, AccountsResponse, HdStoreImportParam,
        KeystoreCommonAccountsParam, KeystoreCommonDeriveParam, KeystoreCommonExportResult,
        PublicKeyParam, PublicKeyResult, SignParam, TcxAction, WalletKeyParam, WalletResult,
    },
    handler::hd_store_import,
};

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut stream = TcpStream::connect("127.0.0.1:9999").unwrap();
    stream.set_nodelay(true).expect("set_nodelay call failed");

    let wallet = WalletKeyParam {
        id: args[1].clone(),
        password: args[2].clone(),
    };
    let wallet_msg = encode_message(wallet).unwrap();
    let action = TcxAction {
        method: "export_mnemonic".to_string(),
        param: Some(prost_types::Any {
            type_url: "deeper".to_string(),
            value: wallet_msg,
        }),
    };
    println!(
        "write result {:?}",
        stream.write(&encode_message(action).unwrap())
    );
    let _ = stream.shutdown(std::net::Shutdown::Write);
    let mut buf = vec![];
    println!("read result {:?}", stream.read_to_end(&mut buf));
    let export_result: KeystoreCommonExportResult =
        KeystoreCommonExportResult::decode(buf.as_slice()).unwrap();
    println!("export_result {:?}", export_result);
}
