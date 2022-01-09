use data_encoding::BASE64;
use ring::rand::SecureRandom;
use ring::{pbkdf2, rand};
use std::num::NonZeroU32;

const SALT_BYTE_SIZE: usize = 32;
const HASH_BYTE_SIZE: usize = 64;
const PBKDF2_ITERATIONS: u32 = 10000;
const DIGEST_ALGORITHM: &str = "sha512";

pub fn verify(candidate: &str, salt: &str, hash: &str) -> bool {
    let extracted_salt = extract_salt(salt);
    let hash_bytes = BASE64.decode(hash.as_bytes()).unwrap();

    let verified = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &extracted_salt.as_bytes(),
        candidate.as_bytes(),
        &hash_bytes,
    );

    return verified.is_ok();
}

pub fn generate_salt_and_hash(password: &str) -> (String, String) {
    let rng = rand::SystemRandom::new();

    // Gen salt
    let mut salt = [0u8; SALT_BYTE_SIZE];
    rng.fill(&mut salt).unwrap();
    let salt_base64 = BASE64.encode(&salt);

    // Gen hash
    let mut hash = [0u8; HASH_BYTE_SIZE];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &salt_base64.as_bytes(),
        password.as_bytes(),
        &mut hash,
    );

    // Convert salt to a joined format
    let salt_joined = join_salt(&salt_base64);

    return (salt_joined, BASE64.encode(&hash));
}

fn join_salt(salt_base64: &str) -> String {
    // vec![&str, &str, &str].join("!")
    // To make a String into a slice &'a str:
    // let s: String = "abcde".to_owned();
    // let s_slice: &str = &s[..];
    return vec![
        DIGEST_ALGORITHM,
        &PBKDF2_ITERATIONS.to_string()[..],
        salt_base64,
    ]
    .join("!");
}

fn extract_salt(salt: &str) -> String {
    let parts = salt.split("!").collect::<Vec<&str>>();
    let salt = (&parts[2]).to_string();
    return salt;
}
