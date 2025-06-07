use anyhow::Result;
use sled::Db;

use crate::credentials::dto::Credentials;

pub fn get_credentials(username: &str, db: Db) -> Option<Credentials> {
    let bytes_op = db.get(username).unwrap();

    if let Some(bytes) = bytes_op {
        let credentials: Credentials = serde_json::from_slice(&bytes).unwrap();
        Some(credentials)
    } else {
        None
    }
}

pub fn save_credentials(username: &str, credentials: Credentials, db: Db) -> Result<()> {
    let bytes = serde_json::to_vec(&credentials).unwrap();
    db.insert(username, bytes).map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
}
