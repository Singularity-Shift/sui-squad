use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub telegram_id: String,
    pub max_epoch: u64,
    pub public_key: String,
    pub randomness: String,
}

#[derive(Debug, Deserialize)]
pub struct BalanceObject {
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "hasPublicTransfer")]
    pub has_public_transfer: bool,
    pub fields: BalanceFields,
}

#[derive(Debug, Deserialize)]
pub struct BalanceFields {
    pub id: IdField,
    pub name: NameField,
    pub value: ValueField,
}

#[derive(Debug, Deserialize)]
pub struct IdField {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct NameField {
    #[serde(rename = "type")]
    pub type_field: String,
    pub fields: DummyFields,
}

#[derive(Debug, Deserialize)]
pub struct DummyFields {
    pub dummy_field: bool,
}

#[derive(Debug, Deserialize)]
pub struct ValueField {
    #[serde(rename = "type")]
    pub type_field: String,
    pub fields: CoinFields,
}

#[derive(Debug, Deserialize)]
pub struct CoinFields {
    pub balance: String,
    pub id: IdField,
}

impl From<(String, u64, String, String)> for State {
    fn from(state: (String, u64, String, String)) -> Self {
        let (telegram_id, max_epoch, public_key, randomness) = state;

        Self {
            telegram_id,
            max_epoch,
            public_key,
            randomness,
        }
    }
}
