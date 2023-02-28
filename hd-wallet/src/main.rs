use bytes::BytesMut;
use prost::Message;
use std::env;
use std::ffi::CString;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::raw::c_char;
use tcx::api_json::{Response, TcxAction};
use tcx::{encode_message, handler};
//mod main2;

fn _to_c_char(str: &str) -> *const c_char {
    CString::new(str).unwrap().into_raw()
}

fn handle_input_msg(input: String) {
    //println!("recv handle_input_msg {:?}", input);
    let action: TcxAction = serde_json::from_str(&input).expect("decode tcx api");
    let res = match action.method.to_lowercase().as_str() {
        "init_token_core_x" => {
            handler::init_token_core_x(&action.param.to_string()).map(|_| String::new())
        }
        "scan_keystores" => handler::scan_keystores().map(|_| String::new()),
        "hd_store_create" => {
            handler::hd_store_create(&action.param.to_string()).map(|_| String::new())
        }
        "hd_store_import" => handler::hd_store_import(&action.param.to_string()),
        "hd_store_export" => {
            handler::hd_store_export(&action.param.to_string()).map(|_| String::new())
        }
        "export_mnemonic" => handler::export_mnemonic(&action.param.to_string()),
        "keystore_common_derive" => handler::keystore_common_derive(&action.param.to_string()),

        "private_key_store_import" => handler::private_key_store_import(&action.param.to_string()),
        "private_key_store_export" => handler::private_key_store_export(&action.param.to_string()),
        "export_private_key" => handler::export_private_key(&action.param.to_string()),
        "keystore_common_verify" => handler::keystore_common_verify(&action.param.to_string()),
        "keystore_common_delete" => handler::keystore_common_delete(&action.param.to_string()),
        "keystore_common_exists" => handler::keystore_common_exists(&action.param.to_string()),
        "keystore_common_accounts" => handler::keystore_common_accounts(&action.param.to_string()),

        "sign_tx" => handler::sign_tx(&action.param.to_string()),
        "get_public_key" => handler::get_public_key(&action.param.to_string()),

        "tron_sign_msg" => handler::tron_sign_message(&action.param.to_string()),

        "substrate_keystore_exists" => {
            handler::substrate_keystore_exists(&action.param.to_string())
        }

        "substrate_keystore_import" => {
            handler::import_substrate_keystore(&action.param.to_string())
        }

        "substrate_keystore_export" => {
            handler::export_substrate_keystore(&action.param.to_string())
        }

        // !!! WARNING !!! used for `cache_dk` feature
        "get_derived_key" => handler::get_derived_key(&action.param.to_string()),
        // !!! WARNING !!! used for test only
        "unlock_then_crash" => handler::unlock_then_crash(&action.param.to_string()),
        _ => Err(anyhow::format_err!("unsupported_method")),
    };

    match res {
        Ok(res) => {
            if res.is_empty() {
                let response = Response {
                    is_success: true,
                    error: "Ok".to_string(),
                };
                println!("{:?}", serde_json::to_string(&response))
            } else {
                println!("{:?}", res);
            }
        }
        Err(e) => {
            let response = Response {
                is_success: false,
                error: format!("{:?}", e),
            };
            println!("{:?}", serde_json::to_string(&response))
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _ = handler::scan_keystores();
    handle_input_msg(args[2].clone());
    // let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    // for stream in listener.incoming() {
    //     std::thread::spawn(|| handle_client(stream.unwrap()));
    // }

    //main2::func(_args.clone());
    // main3::func(args);
}
