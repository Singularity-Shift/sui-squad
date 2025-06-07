use squad_connect::client::squad_connect::SquadConnect;
use std::path::PathBuf;
use sui_sdk::types::base_types::SuiAddress;

#[derive(Clone)]
pub struct KeeperState {
    squad_connect_client: SquadConnect,
    admin: SuiAddress,
    path: PathBuf,
}

impl From<(SquadConnect, SuiAddress, PathBuf)> for KeeperState {
    fn from(state: (SquadConnect, SuiAddress, PathBuf)) -> Self {
        let (squad_connect_client, admin, path) = state;

        Self {
            squad_connect_client,
            admin,
            path,
        }
    }
}

impl KeeperState {
    /// Get a reference to the squad connect client
    pub fn squad_connect_client(&self) -> &SquadConnect {
        &self.squad_connect_client
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
