use std::{env, fs, path::PathBuf};

use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::types::{base_types::SuiAddress, crypto::SignatureScheme};

pub fn get_account() -> (SuiAddress, PathBuf) {
    let seed = env::var("SEED").expect("SEED must be set");
    let seed = seed.trim_matches('"');

    let keystore_path = env::var("KEYSTORE_PATH").expect("KEYSTORE_PATH must be set");

    let path = PathBuf::from(keystore_path);

    // Ensure the keystore directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create keystore directory");
    }

    // Create empty keystore file if it doesn't exist
    if !path.exists() {
        fs::write(&path, "[]").expect("Failed to create keystore file");
    }

    let mut file_keystore = FileBasedKeystore::new(&path).unwrap_or_else(|e| {
        eprintln!("Failed to create keystore at {:?}: {}", path, e);
        eprintln!("Make sure the keystore directory exists and has proper permissions");
        panic!("Keystore initialization failed");
    });

    let sender = file_keystore
        .import_from_mnemonic(seed, SignatureScheme::ED25519, None, None)
        .expect("Failed to import from mnemonic");

    println!("Sender: {}", sender);

    (sender, path)
}
