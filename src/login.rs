use std::{io::{Read, Write}, net::TcpListener};
use rand::Rng;
use base64::{prelude::BASE64_STANDARD as b64, Engine};
use urlencoding::encode as urlencode;

use crate::errors::FAILED_TO_START_LISTENER;

pub fn login() -> i32 {
    let salt = generate_salt();
    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type={}&scope={}&code_challenge={}&code_challenge_method={}", 
        "598611254890-dsmr3ukd71o2smsdp8ducoi1tdquaugn.apps.googleusercontent.com",
        urlencode("http://127.0.0.1:3000").into_owned(),
        "code",
        urlencode("https://www.googleapis.com/auth/drive").into_owned(),
        urlencode(&salt).into_owned(),
        "plain"
    );

    println!("Please copy the following link in your browser:\n\n{}", url);

    let listener_res = TcpListener::bind("127.0.0.1:3000");
    if listener_res.is_err() {
        let e = listener_res.err().unwrap();
        eprintln!("Failed to attach to port '3000'. {e}");
        return FAILED_TO_START_LISTENER;
    }

    let listener = listener_res.unwrap();
    let mut content = String::new();
    for stream in listener.incoming() {
        match stream {
             Err(e) => {
                eprintln!("Failed to accept listener {e}");
                return FAILED_TO_START_LISTENER;
            }
            Ok(mut stream) => {
                let mut buff = [0;8];
                loop {
                    let r = stream.read(&mut buff).unwrap();
                    let str = String::from_utf8(buff.to_vec()).unwrap();

                    content += &str;
                    buff = [0;8];

                    if r == 8 {
                        continue;
                    }
                    let response = b"HTTP/1.1 200 OK\r\r";
                    let _ = stream.write_all(response);

                    break;
                }

            }
        };
        break;
    };
    println!("{content}");

    0
}

fn generate_salt() -> String {
    let salt: String = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    b64.encode(salt.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{prelude::BASE64_STANDARD as b64, Engine};

    #[test]
    fn should_generate_an_alphanumerical_salt() {
        let salt = generate_salt();
        let bytes = b64.decode(salt).unwrap();
        let decoded = String::from_utf8(bytes).unwrap();

        for c in decoded.chars() {
            assert!(c.is_alphanumeric());
        }
    }
}
