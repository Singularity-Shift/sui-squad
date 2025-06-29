use std::str::FromStr;
use std::{env, sync::Arc};

use axum::extract::{Extension, Json, State};
use shared_crypto::intent::Intent;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::types::TypeTag;
use sui_sdk::{
    json::SuiJsonValue,
    rpc_types::{EventFilter, SuiTransactionBlockResponseOptions, SuiTypeTag},
    types::{
        base_types::ObjectID, quorum_driver_types::ExecuteTransactionRequestType,
        transaction::Transaction,
    },
};
use sui_squad_core::helpers::dtos::UserPayload;
use sui_squad_core::{
    helpers::dtos::{DigestResponse, WithdrawRequest},
    package::dto::Event,
};

use crate::{error::ErrorKeeper, state::KeeperState};

pub async fn withdraw(
    State(keeper_state): State<Arc<KeeperState>>,
    Extension(user): Extension<UserPayload>,
    Json(withdraw_request): Json<WithdrawRequest>,
) -> Result<Json<DigestResponse>, ErrorKeeper> {
    let package_id = env::var("SUI_SQUAD_PACKAGE_ID").map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let package_object_id = ObjectID::from_hex_literal(&package_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let squad_connect_client = keeper_state.squad_connect_client();

    let node = squad_connect_client.get_node();

    let admin = keeper_state.admin();
    let path = keeper_state.path();

    let keystore = FileBasedKeystore::new(&path).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let admin_events = node
        .event_api()
        .query_events(
            EventFilter::MoveEventType(Event::AdminEvent.to_string().parse().unwrap()),
            None,
            None,
            false,
        )
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let admin_event = admin_events
        .data
        .iter()
        .find(|event| {
            if let Some(wallet) = event.parsed_json.get("wallet") {
                if let Some(wallet_str) = wallet.as_str() {
                    return wallet_str == admin.to_string();
                }
            }
            false
        })
        .ok_or_else(|| ErrorKeeper {
            message: "Admin not found".to_string(),
            status: 404,
        })?;

    let admin_id = admin_event
        .parsed_json
        .get("admin_id")
        .ok_or_else(|| ErrorKeeper {
            message: "Admin not found".to_string(),
            status: 404,
        })?
        .as_str()
        .ok_or_else(|| ErrorKeeper {
            message: "Admin id is not a string".to_string(),
            status: 404,
        })?;

    let admin_object_id = ObjectID::from_hex_literal(admin_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let account_events = node
        .event_api()
        .query_events(
            EventFilter::MoveEventType(Event::AccountEvent.to_string().parse().unwrap()),
            None,
            None,
            false,
        )
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let account_event = account_events
        .data
        .iter()
        .find(|event| {
            if let Some(telegram_id) = event.parsed_json.get("telegram_id") {
                if let Some(telegram_id_str) = telegram_id.as_str() {
                    return telegram_id_str == user.telegram_id;
                }
            }
            false
        })
        .ok_or_else(|| ErrorKeeper {
            message: "Account not found".to_string(),
            status: 404,
        })?;

    let account_id = account_event
        .parsed_json
        .get("account_id")
        .ok_or_else(|| ErrorKeeper {
            message: "Account id not found".to_string(),
            status: 404,
        })?
        .as_str()
        .ok_or_else(|| ErrorKeeper {
            message: "Account id is not a string".to_string(),
            status: 404,
        })?;

    let account_object_id = ObjectID::from_hex_literal(account_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let coin_name = "0x2::sui::SUI".to_string();

    let coin_type = TypeTag::from_str(&coin_name).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let address = &withdraw_request.address;

    let address_object_id = ObjectID::from_address(address.parse().unwrap());

    let tx = node
        .transaction_builder()
        .move_call(
            admin.clone(),
            package_object_id,
            "account",
            "withdraw",
            vec![SuiTypeTag::from(coin_type)],
            vec![
                SuiJsonValue::from_object_id(account_object_id),
                SuiJsonValue::from_object_id(admin_object_id),
                SuiJsonValue::new(serde_json::Value::String(
                    withdraw_request.amount.to_string(),
                ))
                .map_err(|e| ErrorKeeper {
                    message: e.to_string(),
                    status: 500,
                })?,
                SuiJsonValue::from_object_id(address_object_id),
            ],
            None,
            10_000_000,
            None,
        )
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let signature = keystore
        .sign_secure(&admin, &tx, Intent::sui_transaction())
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let transaction_response = node
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx.clone(), vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await
        .unwrap();

    println!("{}", transaction_response);
    println!("Transaction created successfully: {:?}", tx);

    Ok(Json(DigestResponse {
        digest: transaction_response.digest.to_string(),
    }))
}
