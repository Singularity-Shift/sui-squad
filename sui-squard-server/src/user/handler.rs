use std::{env, sync::Arc};

use axum::{extract::State, Json};
use shared_crypto::intent::Intent;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{json::SuiJsonValue, rpc_types::{EventFilter, 
    SuiTransactionBlockResponseOptions}, types::{base_types::ObjectID, quorum_driver_types::ExecuteTransactionRequestType, transaction::{Argument, Transaction}}};
use sui_squad_core::{helpers::dtos::UserPayload, package::dto::Event};

use crate::state::KeeperState;

#[axum::debug_handler]
pub async fn create_user(State(keeper_state): State<Arc<KeeperState>>, user: Json<UserPayload>) {
    let package_id = env::var("SUI_SQUARD_PACKAGE_ID").expect("SUI_SQUARD_PACKAGE_ID is not set");

    let node = keeper_state.squard_connect_client().get_node();

    let admin = keeper_state.admin();


    let events = node.event_api()
        .query_events(
            EventFilter::MoveEventType(Event::AdminEvent.to_string().parse().unwrap()),
            None,
            None,
            false
        )
        .await
        .unwrap();

    let admin_event = events.data.iter().find(|event| {
        if let Some(wallet) = event.parsed_json.get("wallet") {
            if let Some(wallet_str) = wallet.as_str() {
                return wallet_str == admin.to_string();
            }
        }
        false
    });

    // let admin_id = admin_event.unwrap().parsed_json.get("admin_id").unwrap().as_str().unwrap();

    // let package_object = ObjectID::from_hex_literal(&package_id).unwrap();

    // let admin_object_id = ObjectID::from_hex_literal(admin_id).unwrap();

    // let relations_id = ObjectID::from_bytes(&[]).unwrap();

    // let user_address = ObjectID::from_address(user.wallet.parse().unwrap());

    // println!("package_object: {:?}", package_object);

    // println!("admin_object_id: {:?}", admin_object_id);

    // println!("user_address: {:?}", user_address);

    // let tx = node.transaction_builder().move_call(
    //     *admin,
    //     package_object,
    //     "admin",
    //     "set_relations",
    //     vec![],
    //     vec![
    //         SuiJsonValue::from_object_id(admin_object_id),       // Admin object
    //         SuiJsonValue::from_object_id(relations_id),               // relations_id_opt: Option<ID> (None as empty vector)
    //         SuiJsonValue::from_bcs_bytes(None, user.telegram_id.as_bytes()).unwrap(), // telegram_id: String
    //         SuiJsonValue::from_object_id(user_address), // user: address
    //     ],
    //     None,
    //     1000000,
    //     None
    // ).await;

    // if let Ok(tx) = tx {
    //     let keystore = FileBasedKeystore::new(keeper_state.path()).expect("Failed to create keystore");
    //     let signature = keystore.sign_secure(admin, &tx, Intent::sui_transaction()).unwrap();

    //     print!("Executing the transaction...");
    //     let transaction_response = node.quorum_driver_api()
    //     .execute_transaction_block(
    //         Transaction::from_data(tx.clone(), vec![signature]),
    //         SuiTransactionBlockResponseOptions::full_content(),
    //         Some(ExecuteTransactionRequestType::WaitForLocalExecution),
    //     )
    //     .await.unwrap();

    //     println!("{}", transaction_response);
    //     println!("Transaction created successfully: {:?}", tx);
    // } else {
    //     println!("Error creating transaction: {:?}", tx.err());
    // }

    
}