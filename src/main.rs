use osm_pwdhash::{generate_salt_and_hash, verify};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("error: must provide a password in the argument");
        eprintln!("usage: osm-pwdhash your-password");
        process::exit(1);
    }
    let password = &args[1];

    let (salt, hash) = generate_salt_and_hash(password);
    println!("pawd: {}", password);
    println!("salt: {}", salt);
    println!("hash: {}", hash);

    let verified = verify(password, &salt, &hash);
    println!("verified: {}", verified);
}
