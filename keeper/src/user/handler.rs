use std::{env, sync::Arc};

use axum::{Json, extract::State, response::Result};
use shared_crypto::intent::Intent;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore};
use sui_sdk::{
    rpc_types::{EventFilter, SuiObjectDataOptions, SuiTransactionBlockResponseOptions},
    types::{
        Identifier,
        base_types::ObjectID,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{Argument, CallArg, Command, ObjectArg, Transaction, TransactionData},
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
    let mut relations_id: Option<String> = None;

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

    if let Some(relation) = relation_event {
        relations_id = Some(
            relation
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
                .to_string(),
        );

        let relations = relation
            .parsed_json
            .get("relations")
            .ok_or_else(|| ErrorKeeper {
                message: "Relations not found".to_string(),
                status: 404,
            })?;
        let relations_vector = relations.as_array().ok_or_else(|| ErrorKeeper {
            message: "Relations is not a vector".to_string(),
            status: 404,
        })?;

        let relation_str = relations_vector
            .get(0)
            .ok_or_else(|| ErrorKeeper {
                message: "Relation is not a string".to_string(),
                status: 404,
            })?
            .as_str()
            .ok_or_else(|| ErrorKeeper {
                message: "Relation is not a string".to_string(),
                status: 404,
            })?;
        if relation_str == user.telegram_id {
            return Ok(());
        }
    }

    let coins = node
        .coin_read_api()
        .get_coins(*admin, None, None, None)
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let coin = coins.data.into_iter().next().unwrap();

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

    let package_object = ObjectID::from_hex_literal(&package_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;
    let module = Identifier::new("admin").map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;
    let function = Identifier::new("set_relations").map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let admin_object_id = ObjectID::from_hex_literal(admin_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    // Fetch the admin object to get the full ObjectRef
    let admin_object = node
        .read_api()
        .get_object_with_options(admin_object_id, SuiObjectDataOptions::new())
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let admin_object_ref = admin_object
        .data
        .ok_or_else(|| ErrorKeeper {
            message: "Admin object not found".to_string(),
            status: 404,
        })?
        .object_ref();

    let mut ptb = ProgrammableTransactionBuilder::new();

    ptb.input(CallArg::Object(ObjectArg::SharedObject {
        id: admin_object_ref.0,
        initial_shared_version: admin_object_ref.1,
        mutable: true,
    }))
    .map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    if let Some(relations_id) = relations_id {
        let relations_id_object =
            ObjectID::from_hex_literal(&relations_id).map_err(|e| ErrorKeeper {
                message: e.to_string(),
                status: 500,
            })?;

        let relations_id_object_ref = node
            .read_api()
            .get_object_with_options(relations_id_object, SuiObjectDataOptions::new())
            .await
            .map_err(|e| ErrorKeeper {
                message: e.to_string(),
                status: 500,
            })?;

        let relations_id_object_ref_data =
            relations_id_object_ref.data.ok_or_else(|| ErrorKeeper {
                message: "Relations id object not found".to_string(),
                status: 404,
            })?;

        println!(
            "Relations id object ref data: {:?}",
            relations_id_object_ref_data
        );

        let relations_id_object_ref_data_ref = relations_id_object_ref_data.object_ref();

        ptb.input(CallArg::Object(ObjectArg::SharedObject {
            id: relations_id_object_ref_data_ref.0,
            initial_shared_version: relations_id_object_ref_data_ref.1,
            mutable: true,
        }))
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;
    } else {
        ptb.input(CallArg::Pure(vec![])).map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;
    }

    ptb.input(CallArg::Pure(user.telegram_id.as_bytes().to_vec()))
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;
    ptb.input(CallArg::Pure(user.wallet.as_bytes().to_vec()))
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    ptb.command(Command::move_call(
        package_object,
        module,
        function,
        vec![],
        vec![
            Argument::Input(0),
            Argument::Input(1),
            Argument::Input(2),
            Argument::Input(3),
        ],
    ));

    let builder = ptb.finish();

    let gas_budget = 10_000_000;

    let gas_price = node
        .read_api()
        .get_reference_gas_price()
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;
    let tx = TransactionData::new_programmable(
        *admin,
        vec![coin.object_ref()],
        builder,
        gas_budget,
        gas_price,
    );

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
