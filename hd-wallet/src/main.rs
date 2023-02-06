use bytes::BytesMut;
use prost::Message;
use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
//use tcx::api::InitTokenCoreXParam;
//use tcx::{call_tcx_api, get_last_err_message};

//use tcx_constants;
use tcx_bch;
use tcx_primitive;
use tcx_substrate;

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

        //————————————Substrate———————————//
        "substrate_pk_from_seed" => println!("public key from seed: {}", tcx_primitive::recover_pk_from_seed(&args[2])),
        "substrate_pk_from_mnemonic" => println!("public key from mnemonic: {}", tcx_primitive::recover_pk_from_mnemonic(&args[2])),
        "substrate_sk_drive_from_mnemonic" => println!("private key from mnemonic: {}", tcx_primitive::recover_sk_drive_from_mnemoic(&args[2], &args[3])),
        "substrate_ss58" => println!("ss58 address from mnemonic: {}", tcx_primitive::recover_address_from_mnemonic(&args[2])),
        "substrate_pk_drive_from_mnemonic" => println!("public key from mnemonic: {}", tcx_primitive::recover_pk_drive_from_mnemoic(&args[2], &args[3])),
        "export_from_secret_key" => println!("export_from_secret_key: {:?}", tcx_substrate::export_from_secret_key()),
        
        //————————————BCH———————————//
        "bip39_mnemonic_convert_seed"  => println!("bip39 mnemonic convert seed: {:?} ", tcx_primitive::mnemonic_convert_seed(&args[2])),
        "bch_address_from_pub_key" => println!("bch_address from pub key: {}", tcx_bch::bch_address_from_pub_key(&args[2])),
        "bch_address_from_pri_key" => println!("bch_address from pri key: {}", tcx_bch::bch_address_from_pri_key(&args[2])),
        "bch_sign_tx" => println!("bch sign transaction: {}", tcx_bch::bch_sign_to_tx(&args[2], args[3].parse::<i64>().unwrap(), &args[4])),
        "bch_account_create" => println!("keystore info: {}", tcx_bch::bch_account_create(&args[2])),
        "bch_account_recover" => println!("keystore info: {}", tcx_bch::bch_account_recover(&args[2], &args[3])),

        //————————————BTC———————————//
        

        _ => println!("bash:command not found!"),
    }
}
