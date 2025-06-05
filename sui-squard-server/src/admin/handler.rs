use std::{env, path::PathBuf};

use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::types::{base_types::SuiAddress, crypto::SignatureScheme};

pub fn get_account() -> (SuiAddress, PathBuf) {
    let seed = env::var("SEED").expect("SEED must be set");
    let seed = seed.trim_matches('"');

    let mut path = PathBuf::new();
    path.push("sui.keystore");

    let mut file_keystore = FileBasedKeystore::new(&path).expect("Failed to create keystore");

    let sender = file_keystore
        .import_from_mnemonic(seed, SignatureScheme::ED25519, None, None)
        .expect("Failed to import from mnemonic");

    println!("Sender: {}", sender);

    (sender, path)
}
