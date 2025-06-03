use squard_connect::client::squard_connect::SquardConnect;
use std::path::PathBuf;
use sui_sdk::types::base_types::SuiAddress;

#[derive(Clone)]
pub struct KeeperState {
    squard_connect_client: SquardConnect,
    admin: SuiAddress,
    path: PathBuf,
}

impl From<(SquardConnect, SuiAddress, PathBuf)> for KeeperState {
    fn from(state: (SquardConnect, SuiAddress, PathBuf)) -> Self {
        let (squard_connect_client, admin, path) = state;

        Self {
            squard_connect_client,
            admin,
            path,
        }
    }
}

impl KeeperState {
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
