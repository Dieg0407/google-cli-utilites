use rand::Rng;
use base64::{prelude::BASE64_STANDARD as b64, Engine};

pub fn login() -> i32 {
    println!("Encoded salt: {}", generate_salt());
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
