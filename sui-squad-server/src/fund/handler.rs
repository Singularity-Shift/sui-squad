use std::{env, str::FromStr, sync::Arc};

use axum::{
    extract::State,
    http::HeaderMap,
    response::{Json, Result},
};

use sui_sdk::{
    json::SuiJsonValue,
    rpc_types::{EventFilter, SuiTransactionBlockResponseOptions, SuiTypeTag},
    types::{
        TypeTag,
        base_types::{ObjectID, SuiAddress},
        crypto::PublicKey,
        quorum_driver_types::ExecuteTransactionRequestType,
    },
};

use crate::{error::ErrorKeeper, state::KeeperState};

use super::dto::FundRequest;
use sui_squad_core::{helpers::dtos::DigestResponse, package::dto::Event};

#[utoipa::path(
    post,
    path = "/fund",
    summary = "Get user auth info",
    description = "Get user auth info",
    request_body = [FundRequest],
    responses(
        (status = 201, description = "Get user auth info", body = [FundRequest])
    )
)]
#[axum::debug_handler]
pub async fn fund(
    State(keeper_state): State<Arc<KeeperState>>,
    headers: HeaderMap,
    Json(fund_request): Json<FundRequest>,
) -> Result<Json<DigestResponse>, ErrorKeeper> {
    let package_id = env::var("SUI_SQUAD_PACKAGE_ID").expect("SUI_SQUAD_PACKAGE_ID is not set");

    let node = keeper_state.squard_connect_client().get_node();

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

    let account_event = account_events.data.iter().find(|event| {
        if let Some(telegram_id) = event.parsed_json.get("telegram_id") {
            if let Some(telegram_id_str) = telegram_id.as_str() {
                return telegram_id_str == fund_request.telegram_id;
            }
        }
        false
    });

    let account_event = account_event.ok_or_else(|| ErrorKeeper {
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
            message: "Admin id is not a string".to_string(),
            status: 404,
        })?;

    let node = keeper_state.squard_connect_client().get_node();
    let path = keeper_state.path();

    let jwt = headers.get("Authorization").ok_or_else(|| ErrorKeeper {
        message: "Authorization header not found".to_string(),
        status: 401,
    })?;

    let jwt = jwt
        .to_str()
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 401,
        })?
        .split(" ")
        .nth(1)
        .ok_or_else(|| ErrorKeeper {
            message: "JWT is not valid".to_string(),
            status: 401,
        })?;

    let mut squard_connect_client = keeper_state.squard_connect_client().clone();

    squard_connect_client.set_jwt(jwt.to_string());

    squard_connect_client.set_zk_proof_params(
        fund_request.randomness,
        fund_request.public_key.clone(),
        fund_request.max_epoch,
    );

    let account = squard_connect_client
        .get_address()
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let zk_login_inputs = squard_connect_client
        .recover_seed_address()
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let sender = SuiAddress::from_str(&account.address).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let package_object_id = ObjectID::from_hex_literal(&package_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let account_id_object_id = ObjectID::from_hex_literal(account_id).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let coin_name = "0x2::sui::SUI".to_string();

    let coins = node
        .coin_read_api()
        .get_coins(sender, Some(coin_name.clone()), None, None)
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let coin = coins.data.first().ok_or_else(|| ErrorKeeper {
        message: "Coin not found".to_string(),
        status: 404,
    })?;

    let coin_id = coin.coin_object_id;

    let coin_type = TypeTag::from_str(&coin_name).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let tx = node
        .transaction_builder()
        .move_call(
            sender,
            package_object_id,
            "account",
            "fund",
            vec![SuiTypeTag::from(coin_type)],
            vec![
                SuiJsonValue::from_object_id(account_id_object_id),
                SuiJsonValue::new(serde_json::Value::String(fund_request.telegram_id.clone()))
                    .map_err(|e| ErrorKeeper {
                        message: e.to_string(),
                        status: 500,
                    })?,
                SuiJsonValue::from_object_id(coin_id),
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

    let signer_pk = PublicKey::from_str(&fund_request.public_key).map_err(|e| ErrorKeeper {
        message: e.to_string(),
        status: 500,
    })?;

    let signer_address = SuiAddress::from(&signer_pk);

    let transaction = squard_connect_client
        .sign_transaction(
            tx.clone(),
            signer_address,
            zk_login_inputs,
            fund_request.max_epoch,
            path.clone(),
        )
        .await
        .map_err(|e| ErrorKeeper {
            message: e.to_string(),
            status: 500,
        })?;

    let transaction_response = node
        .quorum_driver_api()
        .execute_transaction_block(
            transaction,
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await
        .unwrap();

    println!("transaction_response: {:?}", transaction_response);
    println!("Transaction created successfully: {:?}", tx);

    Ok(Json(DigestResponse {
        digest: transaction_response.digest.to_string(),
    }))
}
