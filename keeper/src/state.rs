use sled::Db;
use squard_connect::client::squard_connect::SquardConnect;

#[derive(Clone)]
pub struct KeeperState {
    db: Db,
    squard_connect_client: SquardConnect,
}

impl From<(Db, SquardConnect)> for KeeperState {
    fn from(state: (Db, SquardConnect)) -> Self {
        let (db, squard_connect_client) = state;

        Self { db, squard_connect_client }
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
}