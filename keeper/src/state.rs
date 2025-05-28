use std::path::PathBuf;

use sled::Db;
use squard_connect::client::squard_connect::SquardConnect;
use sui_sdk::types::base_types::SuiAddress;

#[derive(Clone)]
pub struct KeeperState {
    db: Db,
    squard_connect_client: SquardConnect,
    admin: SuiAddress,
    path: PathBuf,
}

impl From<(Db, SquardConnect, SuiAddress, PathBuf)> for KeeperState {
    fn from(state: (Db, SquardConnect, SuiAddress, PathBuf)) -> Self {
        let (db, squard_connect_client, admin, path) = state;

        Self { db, squard_connect_client, admin, path }
    }
}

impl KeeperState {
    /// Get a reference to the database
    pub fn db(&self) -> &Db {
        &self.db
    }
    
    /// Get a reference to the squard connect client
    pub fn squard_connect_client(&self) -> &SquardConnect {
        &self.squard_connect_client
    }

    /// Get a reference to the admin
    pub fn admin(&self) -> &SuiAddress {
        &self.admin
    }

    /// Get a reference to the path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}