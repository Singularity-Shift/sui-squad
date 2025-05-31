use std::{env, path::PathBuf};

use anyhow::Result;
use fastcrypto_zkp::bn254::zk_login::ZkLoginInputs;
use sui_sdk::{
    SuiClient,
    json::SuiJsonValue,
    rpc_types::{EventFilter, SuiTransactionBlockResponseOptions},
    types::{
        base_types::{ObjectID, SuiAddress},
        quorum_driver_types::ExecuteTransactionRequestType,
        utils::sign_zklogin_tx_with_default_proof,
    },
};
use sui_squad_core::package::dto::Event;

pub async fn create_account_if_not_exists(
    sender: SuiAddress,
    path: PathBuf,
    node: &SuiClient,
) -> Result<String> {
    let package_id = env::var("SUI_SQUARD_PACKAGE_ID").expect("SUI_SQUARD_PACKAGE_ID is not set");

    let account_events = node
        .event_api()
        .query_events(
            EventFilter::MoveEventType(Event::AccountEvent.to_string().parse().unwrap()),
            None,
            None,
            false,
        )
        .await?;

    let account_event = account_events.data.iter().find(|event| {
        if let Some(wallet) = event.parsed_json.get("wallet") {
            if let Some(wallet_str) = wallet.as_str() {
                return wallet_str == sender.to_string();
            }
        }
        false
    });

    if let None = account_event {
        let relation_events = node
            .event_api()
            .query_events(
                EventFilter::MoveEventType(Event::RelationEvent.to_string().parse().unwrap()),
                None,
                None,
                false,
            )
            .await?;

        let relation_event = relation_events.data.last();

        let relation_id = relation_event
            .unwrap()
            .parsed_json
            .get("relation_id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let package_object_id = ObjectID::from_hex_literal(&package_id)?;

        let relation_object_id = ObjectID::from_hex_literal(&relation_id)?;

        let gas_budget = 10_000_000;

        let tx = node
            .transaction_builder()
            .move_call(
                sender,
                package_object_id,
                "account",
                "create_new_account",
                vec![],
                vec![SuiJsonValue::from_object_id(relation_object_id)],
                None,
                gas_budget,
                None,
            )
            .await?;

        let (sender_adreess, transaction, generic_signature) =
            sign_zklogin_tx_with_default_proof(tx.clone(), true);

        println!("Sender: {}", sender_adreess);

        println!("Transaction: {:?}", transaction.tx_signatures());

        let transaction_response = node
            .quorum_driver_api()
            .execute_transaction_block(
                transaction,
                SuiTransactionBlockResponseOptions::full_content(),
                Some(ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await
            .unwrap();

        println!("{}", transaction_response);
        println!("Transaction created successfully: {:?}", tx);

        Ok(transaction_response.to_string())
    } else {
        Ok(account_event
            .unwrap()
            .parsed_json
            .get("account_id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string())
    }
}
