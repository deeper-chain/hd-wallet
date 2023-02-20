use prost::Message;
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
    let mut stream = TcpStream::connect("127.0.0.1:9999").unwrap();
    stream.set_nodelay(true).expect("set_nodelay call failed");

    let import = HdStoreImportParam {
        mnemonic: "sugar car august uniform romance upset burger mesh polar scissors frame mention"
            .to_string(),
        password: "".to_string(),
        source: "MNEMONIC".to_string(),
        name: "test".to_string(),
        password_hint: "".to_string(),
        overwrite: true,
    };
    let import_msg = encode_message(import).unwrap();
    let action = TcxAction {
        method: "hd_store_import".to_string(),
        param: Some(prost_types::Any {
            type_url: "deeper".to_string(),
            value: import_msg,
        }),
    };
    println!(
        "write result {:?}",
        stream.write(&encode_message(action).unwrap())
    );
    let _ = stream.shutdown(std::net::Shutdown::Write);
    let mut buf = vec![];
    println!("read result {:?}", stream.read_to_end(&mut buf));
    let wallet: WalletResult = WalletResult::decode(buf.as_slice()).unwrap();
    println!("wallet imported {:?}", wallet);
}
