use rand::Rng;
use base64::{prelude::BASE64_STANDARD as b64, Engine};

pub fn login() -> i32 {
    let salt: String = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    let salt = b64.encode(salt.as_bytes());
    println!("Encoded salt: {}", salt);

    0
}
