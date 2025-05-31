use std::path::PathBuf;

use sui_keys::keystore::FileBasedKeystore;

pub fn get_keystore(path: PathBuf) -> FileBasedKeystore {
    let file_keystore = FileBasedKeystore::new(&path).expect("Failed to create keystore");

    file_keystore
}
