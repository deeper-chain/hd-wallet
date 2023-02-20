use bytes::BytesMut;
use failure::format_err;
use prost::Message;
use std::env;
use std::ffi::CString;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::raw::c_char;
use tcx::api::{Response, TcxAction};
use tcx::handler;
mod main2;
mod main3;

pub fn encode_message(msg: impl Message) -> Vec<u8> {
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    buf.to_vec()
}

fn _to_c_char(str: &str) -> *const c_char {
    CString::new(str).unwrap().into_raw()
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = Vec::with_capacity(1000);
    let _ = stream.read_to_end(&mut buf);
    println!("recv buf {:?}", buf.len());
    let action: TcxAction = TcxAction::decode(buf.as_slice()).expect("decode tcx api");
    println!("recv action {:?}", action);
    let res = match action.method.to_lowercase().as_str() {
        "init_token_core_x" => {
            handler::init_token_core_x(&action.param.unwrap().value).map(|_| vec![])
        }
        "scan_keystores" => handler::scan_keystores().map(|_| vec![]),
        "hd_store_create" => handler::hd_store_create(&action.param.unwrap().value).map(|_| vec![]),
        "hd_store_import" => handler::hd_store_import(&action.param.unwrap().value),
        "hd_store_export" => handler::hd_store_export(&action.param.unwrap().value).map(|_| vec![]),
        "export_mnemonic" => handler::export_mnemonic(&action.param.unwrap().value),
        "keystore_common_derive" => handler::keystore_common_derive(&action.param.unwrap().value),

        "private_key_store_import" => {
            handler::private_key_store_import(&action.param.unwrap().value)
        }
        "private_key_store_export" => {
            handler::private_key_store_export(&action.param.unwrap().value)
        }
        "export_private_key" => handler::export_private_key(&action.param.unwrap().value),
        "keystore_common_verify" => handler::keystore_common_verify(&action.param.unwrap().value),
        "keystore_common_delete" => handler::keystore_common_delete(&action.param.unwrap().value),
        "keystore_common_exists" => handler::keystore_common_exists(&action.param.unwrap().value),
        "keystore_common_accounts" => {
            handler::keystore_common_accounts(&action.param.unwrap().value)
        }

        "sign_tx" => handler::sign_tx(&action.param.unwrap().value),
        "get_public_key" => handler::get_public_key(&action.param.unwrap().value),

        "tron_sign_msg" => handler::tron_sign_message(&action.param.unwrap().value),

        "substrate_keystore_exists" => {
            handler::substrate_keystore_exists(&action.param.unwrap().value)
        }

        "substrate_keystore_import" => {
            handler::import_substrate_keystore(&action.param.unwrap().value)
        }

        "substrate_keystore_export" => {
            handler::export_substrate_keystore(&action.param.unwrap().value)
        }

        // !!! WARNING !!! used for `cache_dk` feature
        "get_derived_key" => handler::get_derived_key(&action.param.unwrap().value),
        // !!! WARNING !!! used for test only
        "unlock_then_crash" => handler::unlock_then_crash(&action.param.unwrap().value),
        _ => Err(format_err!("unsupported_method")),
    };

    let send_buf = match res {
        Ok(res) => {
            if res.is_empty() {
                let response = Response {
                    is_success: false,
                    error: "Ok".to_string(),
                };
                encode_message(response)
            } else {
                res
            }
        }
        Err(e) => {
            let response = Response {
                is_success: false,
                error: format!("{:?}", e),
            };
            encode_message(response)
        }
    };

    println!("stream send res {:?}", stream.write_all(&send_buf));
    let _ = stream.shutdown(std::net::Shutdown::Write);
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    let _ = handler::scan_keystores();
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    for stream in listener.incoming() {
        std::thread::spawn(|| handle_client(stream.unwrap()));
    }

    // main2::func(args.clone());
    // main3::func(args);
}
