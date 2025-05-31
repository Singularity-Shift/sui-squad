use std::env;

use anyhow::Result;
use fastcrypto::hash::HashFunction;
use fastcrypto::{
    ed25519::Ed25519KeyPair,
    traits::{KeyPair, Signer},
};
use fastcrypto_zkp::bn254::zk_login::ZkLoginInputs;
use rand::{SeedableRng, rngs::StdRng};
use shared_crypto::intent::{Intent, IntentMessage};
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_sdk::types::signature::GenericSignature;
use sui_sdk::types::transaction::Transaction;
use sui_sdk::{
    SuiClient,
    json::SuiJsonValue,
    rpc_types::EventFilter,
    types::{
        base_types::{ObjectID, SuiAddress},
        crypto::{DefaultHash, SuiKeyPair},
        zk_login_authenticator::ZkLoginAuthenticator,
    },
};
use sui_squad_core::{helpers::dtos::User, package::dto::Event};

pub async fn create_account_if_not_exists(
    sender: SuiAddress,
    node: &SuiClient,
    user: User,
    zk_login_inputs: ZkLoginInputs,
) -> Result<String> {
    let package_id = env::var("SUI_SQUARD_PACKAGE_ID").expect("SUI_SQUARD_PACKAGE_ID is not set");
    let sui_key_pair =
        SuiKeyPair::Ed25519(Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32])));

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

        let intent_message = IntentMessage::new(Intent::sui_transaction(), tx.clone());

        let raw_tx = bcs::to_bytes(&intent_message).expect("Failed to serialize intent message");

        let mut hasher = DefaultHash::default();

        hasher.update(raw_tx.clone());

        let digest = hasher.finalize().digest;

        let signature = sui_key_pair.sign(&digest);

        let zk_login_authenticator =
            ZkLoginAuthenticator::new(zk_login_inputs, user.max_epoch, signature);

        let generic_signature = GenericSignature::from(zk_login_authenticator);

        let transaction_response = node
            .quorum_driver_api()
            .execute_transaction_block(
                Transaction::from_generic_sig_data(tx.clone(), vec![generic_signature]),
                SuiTransactionBlockResponseOptions::full_content(),
                Some(ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await
            .unwrap();

        println!("{}", transaction_response);
        println!("Transaction created successfully: {:?}", tx);

        Ok("Transaction created".to_string())
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
