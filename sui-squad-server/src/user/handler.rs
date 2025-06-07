use std::{env, sync::Arc};

use axum::extract::{Request, State};
use shared_crypto::intent::Intent;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    json::SuiJsonValue,
    rpc_types::{EventFilter, SuiTransactionBlockResponseOptions},
    types::{
        base_types::ObjectID, quorum_driver_types::ExecuteTransactionRequestType,
        transaction::Transaction,
    },
};
use sui_squad_core::{helpers::dtos::UserPayload, package::dto::Event};

use crate::{error::ErrorKeeper, state::KeeperState};

#[axum::debug_handler]
pub async fn create_user_if_not_exists(
    State(keeper_state): State<Arc<KeeperState>>,
    req: Request,
) -> Result<(), ErrorKeeper> {
    let user = req
        .extensions()
        .get::<UserPayload>()
        .ok_or_else(|| ErrorKeeper {
            message: "User not found".to_string(),
            status: 404,
        })?;

    let package_id = env::var("SUI_SQUAD_PACKAGE_ID").expect("SUI_SQUAD_PACKAGE_ID is not set");

    let package_object_id = ObjectID::from_hex_literal(&package_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let node = keeper_state.squad_connect_client().get_node();

    let admin = keeper_state.admin();
    let path = keeper_state.path();

    let keystore = FileBasedKeystore::new(&path).map_err(|e| ErrorKeeper {
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

    let admin_event = admin_events.data.iter().find(|event| {
        if let Some(wallet) = event.parsed_json.get("wallet") {
            if let Some(wallet_str) = wallet.as_str() {
                return wallet_str == admin.to_string();
            }
        }
        false
    });

    let admin_id = admin_event
        .unwrap()
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

    let account_event = account_events.data.iter().find(|event| {
        if let Some(telegram_id) = event.parsed_json.get("telegram_id") {
            if let Some(telegram_id_str) = telegram_id.as_str() {
                return telegram_id_str == user.telegram_id;
            }
        }
        false
    });

    if let None = account_event {
        let gas_budget = 10_000_000;

        let tx = node
            .transaction_builder()
            .move_call(
                admin.clone(),
                package_object_id,
                "account",
                "create_new_account",
                vec![],
                vec![
                    SuiJsonValue::from_object_id(admin_object_id),
                    SuiJsonValue::new(serde_json::Value::String(user.telegram_id.clone()))
                        .map_err(|e| ErrorKeeper {
                            message: e.to_string(),
                            status: 500,
                        })?,
                ],
                None,
                gas_budget,
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

        print!("Executing the transaction...");
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
    }
    Ok(())
}
