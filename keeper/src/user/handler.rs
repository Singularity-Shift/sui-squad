use std::{env, str::FromStr, sync::Arc};

use axum::{Json, extract::State, response::Result};
use serde_json;
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
pub async fn create_user(
    State(keeper_state): State<Arc<KeeperState>>,
    user: Json<UserPayload>,
) -> Result<()> {
    let package_id = env::var("SUI_SQUARD_PACKAGE_ID").expect("SUI_SQUARD_PACKAGE_ID is not set");

    let node = keeper_state.squard_connect_client().get_node();

    let admin = keeper_state.admin();

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

    let relation_events = node
        .event_api()
        .query_events(
            EventFilter::MoveEventType(Event::RelationEvent.to_string().parse().unwrap()),
            None,
            None,
            false,
        )
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let relation_event = relation_events.data.last();

    println!("Relation event: {:?}", relation_event);

    let relation_id = relation_event
        .unwrap()
        .parsed_json
        .get("relation_id")
        .ok_or_else(|| ErrorKeeper {
            message: "Relation id not found".to_string(),
            status: 404,
        })?
        .as_str()
        .ok_or_else(|| ErrorKeeper {
            message: "Relation id is not a string".to_string(),
            status: 404,
        })?
        .to_string();

    println!("Relation id: {:?}", relation_id);

    let users: Vec<String> = relation_event
        .unwrap()
        .parsed_json
        .get("users")
        .ok_or_else(|| ErrorKeeper {
            message: "Relations not found".to_string(),
            status: 404,
        })?
        .as_array()
        .ok_or_else(|| ErrorKeeper {
            message: "Relations is not a vector".to_string(),
            status: 404,
        })?
        .iter()
        .map(|user_obj| user_obj.as_str().unwrap().to_string())
        .collect();

    println!("Users: {:?}", users);

    let relation_str = users
        .iter()
        .find(|&telegram_id| telegram_id == &user.telegram_id);

    println!("Relation str: {:?}", relation_str);

    if let Some(relation) = relation_str {
        println!("User already created: {:?}", relation);
        return Ok(());
    };

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

    println!("Admin id: {:?}", admin_id);

    let package_object_id = ObjectID::from_hex_literal(&package_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let admin_object_id = ObjectID::from_hex_literal(admin_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let relation_object_id = ObjectID::from_hex_literal(&relation_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let gas_budget = 10_000_000;

    let tx = node
        .transaction_builder()
        .move_call(
            *admin,
            package_object_id,
            "admin",
            "set_relations",
            vec![],
            vec![
                SuiJsonValue::from_object_id(admin_object_id),
                SuiJsonValue::from_object_id(relation_object_id),
                SuiJsonValue::new(serde_json::Value::String(user.telegram_id.clone())).unwrap(),
                SuiJsonValue::from_str(&user.wallet).unwrap(),
            ],
            None,
            gas_budget,
            None,
        )
        .await
        .map_err(|e| {
            println!("Error creating transaction: {:?}", e);
            ErrorKeeper {
                message: format!("Error creating transaction: {:?}", e),
                status: 500,
            }
        })?;

    println!("Tx: {:?}", tx);

    let keystore = FileBasedKeystore::new(keeper_state.path()).expect("Failed to create keystore");
    let signature = keystore
        .sign_secure(admin, &tx, Intent::sui_transaction())
        .unwrap();

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

    Ok(())
}
