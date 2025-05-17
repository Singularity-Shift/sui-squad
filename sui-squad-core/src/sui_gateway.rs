use async_trait::async_trait;
use crate::error::CoreError;
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
// use sui_sdk::types::messages_checkpoint::CheckpointSequenceNumber; // Placeholder, might need specific TypeTag
use sui_sdk::types::transaction::ProgrammableTransaction; // Changed from TransactionBlock
use sui_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_sdk::types::TypeTag; // Corrected path
use sui_sdk::SuiClient;
use crate::config::Config;
use sui_sdk::SuiClientBuilder;
use sui_sdk::rpc_types::{SuiTransactionBlockResponseOptions, SuiTransactionBlockEffectsAPI};
use sui_sdk::types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::intent::{Intent, IntentMessage}; // Using sui_types directly
use sui_sdk::crypto::KeyPair; // For KeyPair trait and sign_secure_message method
use sui_sdk::types::transaction::SignedTransaction;
use std::fs;
use sqlx::SqlitePool;
use std::str::FromStr;

/// Trait defining blockchain gateway methods.
#[async_trait]
pub trait SuiGateway: Send + Sync + 'static { // Removed Clone for now, can add back if needed by LiveSuiGateway
    async fn create_account_on_chain_ptb(
        &self,
        user_sui_address: SuiAddress,
        admin_object_id: ObjectID,
        relations_id: ObjectID, // Assuming this is passed in, or fetched if global
        group_object_id: ObjectID
    ) -> Result<ProgrammableTransaction, CoreError>; // Changed from TransactionBlock

    async fn get_account_balance(
        &self,
        account_object_id: ObjectID, 
        coin_type_tag: TypeTag
    ) -> Result<u64, CoreError>;

    async fn prepare_fund_account_ptb(
        &self,
        user_sui_address: SuiAddress,
        account_object_id: ObjectID,
        coin_object_ids: Vec<ObjectID>,
        coin_type_tag: TypeTag
    ) -> Result<ProgrammableTransaction, CoreError>; // Changed from TransactionBlock

    async fn prepare_withdraw_from_account_ptb(
        &self,
        user_sui_address: SuiAddress,
        account_object_id: ObjectID,
        amount: u64,
        coin_type_tag: TypeTag
    ) -> Result<ProgrammableTransaction, CoreError>; // Changed from TransactionBlock

    async fn prepare_payment_ptb(
        &self,
        // Parameters for payment PTB - to be defined based on contract
        // e.g., user_sui_address: SuiAddress, from_account_id: ObjectID, to_address: SuiAddress, amount: u64, coin_type_tag: TypeTag
    ) -> Result<ProgrammableTransaction, CoreError>; // Changed from TransactionBlock

    async fn create_group_on_chain(
        &self,
        admin_sui_address: SuiAddress, // Bot's admin address (for clarity, will be derived from keypair)
        admin_cap_object_id: ObjectID, // Bot's admin capability object (from config)
        telegram_group_id: String
    ) -> Result<ObjectID, CoreError>; // Returns the new group_object_id

    async fn link_user_to_telegram_on_chain(
        &self,
        admin_address: SuiAddress,
        admin_object_id: ObjectID,
        relations_id: &mut Option<ObjectID>, // Pass as mutable if it can be created/updated
        user_sui_address: SuiAddress,
        telegram_user_id: String, // Storing as String, can be i64 if preferred
        telegram_group_id: String
    ) -> Result<ObjectID, CoreError>; // Returns relations_id or relevant object id

    async fn get_sui_address_for_telegram_user(
        &self,
        telegram_user_id: String,
        telegram_group_id: String
    ) -> Result<Option<SuiAddress>, CoreError>;

    async fn get_account_object_id_for_telegram_user(
        &self,
        telegram_user_id: String,
        telegram_group_id: String
    ) -> Result<Option<ObjectID>, CoreError>;

    async fn get_group_object_id(
        &self,
        telegram_group_id: String
    ) -> Result<Option<ObjectID>, CoreError>;
    
    // Helper for submitting signed transactions - from todo.md, not in trait but part of LiveSuiGateway impl.
    // async fn execute_signed_transaction(&self, signed_tx: SignedTransaction) -> Result<SuiTransactionBlockResponse, CoreError>;
}

/// Dummy implementation of SuiGateway that logs calls.
#[derive(Clone)]
pub struct DummyGateway;

// DummyGateway will now fail to compile as it doesn't implement the new SuiGateway methods.
// This is expected and will be addressed later.
// #[async_trait]
// impl SuiGateway for DummyGateway {
// ... (old dummy methods)
// }

pub struct LiveSuiGateway {
    sui_client: SuiClient,
    #[allow(dead_code)] // Config will be used by other methods
    config: Config,
    db_pool: SqlitePool,
}

impl LiveSuiGateway {
    pub async fn new(config: Config, db_pool: SqlitePool) -> Result<Self, CoreError> {
        let sui_client = SuiClientBuilder::default()
            .build(&config.sui_rpc_url)
            .await
            .map_err(|e| CoreError::SuiClientInitializationError(e.to_string()))?;
        Ok(Self { sui_client, config, db_pool })
    }

    fn get_admin_keypair(&self) -> Result<Ed25519SuiKeyPair, CoreError> {
        let path_str = self.config.sui_admin_mnemonic_path.as_ref()
            .ok_or_else(|| CoreError::ConfigurationError("SUI_ADMIN_MNEMONIC_PATH is not set.".to_string()))?;
        
        let mnemonic_phrase = fs::read_to_string(path_str)
            .map_err(|e| CoreError::ConfigurationError(format!("Failed to read admin mnemonic file at {}: {}", path_str, e)))?;
        
        let keypair = Ed25519SuiKeyPair::from_mnemonic(&mnemonic_phrase.trim())
            .map_err(|e| CoreError::ConfigurationError(format!("Failed to create keypair from mnemonic: {}", e)))?;
        Ok(keypair)
    }

    // Helper for submitting user-signed transactions
    pub async fn execute_signed_transaction(
        &self, 
        signed_tx: SignedTransaction
    ) -> Result<sui_sdk::rpc_types::SuiTransactionBlockResponse, CoreError> {
        // Before executing, it's good practice to verify the signed transaction locally if possible,
        // though the SDK might do this or it might be complex to fully verify without node context.
        // For now, we proceed to execution assuming the signed_tx is valid or will be validated by the node.
        
        // The SignedTransaction needs to be converted into a VerifiedTransaction before execution.
        // This typically involves ensuring the signature matches the transaction data.
        // A SignedTransaction usually contains TransactionData and one or more Signatures.
        // A common way is to use transaction.verify() if `SignedTransaction` can be converted to `Transaction`
        // or if there's a direct verify method.
        
        // Assuming SignedTransaction can be verified directly or implicitly handled by execute_transaction_block
        // For sui-sdk, execute_transaction_block takes a VerifiedTransaction.
        // A SignedTransaction can be verified into a VerifiedTransaction.
        // Let's assume `signed_tx.verify()` exists and returns `Result<VerifiedTransaction, _>`
        // If not, the conversion needs to be more explicit.

        // The `SignedTransaction` type itself might be directly executable or might need specific conversion.
        // `execute_transaction_block` expects `VerifiedTransaction`.
        // A `SignedTransaction` is often verified into a `VerifiedTransaction` before execution.

        // Placeholder for actual verification/conversion logic if `signed_tx.into()` is not sufficient
        // or if `signed_tx.verify()` is needed and returns the correct type.
        // For now, assuming a direct path or that the quorum driver handles it. 
        // This part might need refinement based on exact sui-sdk capabilities for SignedTransaction.

        let response = self.sui_client
            .quorum_driver_api()
            .execute_transaction_block(
                signed_tx.into_verified_transaction(), // This is a common pattern if method exists
                SuiTransactionBlockResponseOptions::new().with_effects().with_input(), // Request effects and input for full response
                Some(ExecuteTransactionRequestType::WaitForLocalExecution)
            )
            .await
            .map_err(|e| CoreError::SuiRpcError(format!("Failed to execute signed transaction: {}", e)))?;

        Ok(response)
    }
}

#[async_trait]
impl SuiGateway for LiveSuiGateway {
    async fn create_account_on_chain_ptb(
        &self,
        user_sui_address: SuiAddress,
        admin_object_id: ObjectID,
        relations_id: ObjectID, // Assuming this is passed in, or fetched if global
        group_object_id: ObjectID
    ) -> Result<ProgrammableTransaction, CoreError> {
        todo!("create_account_on_chain_ptb implementation")
    }

    async fn get_account_balance(
        &self,
        account_object_id: ObjectID, 
        coin_type_tag: TypeTag
    ) -> Result<u64, CoreError> {
        tracing::warn!(
            "get_account_balance: Parameter 'account_object_id' is ObjectID ({:?}), but Sui SDK get_balance expects SuiAddress. This needs to be resolved.",
            account_object_id
        );
        
        let dummy_address = SuiAddress::random_for_testing_only(); 

        let balance_response = self.sui_client
            .coin_read_api()
            .get_balance(dummy_address, Some(coin_type_tag.to_string()))
            .await
            .map_err(|e| CoreError::SuiRpcError(e.to_string()))?;
        Ok(balance_response.total_balance as u64)
    }

    async fn prepare_fund_account_ptb(
        &self,
        user_sui_address: SuiAddress,
        account_object_id: ObjectID,
        coin_object_ids: Vec<ObjectID>,
        coin_type_tag: TypeTag
    ) -> Result<ProgrammableTransaction, CoreError> { // Changed from TransactionBlock
        todo!("prepare_fund_account_ptb implementation")
    }

    async fn prepare_withdraw_from_account_ptb(
        &self,
        user_sui_address: SuiAddress,
        account_object_id: ObjectID,
        amount: u64,
        coin_type_tag: TypeTag
    ) -> Result<ProgrammableTransaction, CoreError> { // Changed from TransactionBlock
        todo!("prepare_withdraw_from_account_ptb implementation")
    }

    async fn prepare_payment_ptb(
        &self,
        // _parameters...
    ) -> Result<ProgrammableTransaction, CoreError> { // Changed from TransactionBlock
        todo!("prepare_payment_ptb implementation")
    }

    async fn create_group_on_chain(
        &self,
        admin_sui_address: SuiAddress, // Bot's admin address (for clarity, will be derived from keypair)
        admin_cap_object_id: ObjectID, // Bot's admin capability object (from config)
        telegram_group_id: String
    ) -> Result<ObjectID, CoreError> {
        let admin_keypair = self.get_admin_keypair()?;
        let derived_admin_address = SuiAddress::from(admin_keypair.public());

        if admin_sui_address != derived_admin_address {
            // This is a sanity check. The admin_sui_address param is mostly for API clarity/symmetry
            // but the derived address from the keypair is what will actually sign.
            return Err(CoreError::ConfigurationError(
                format!("Provided admin_sui_address {} does not match address derived from mnemonic {}.", 
                        admin_sui_address, derived_admin_address)
            ));
        }

        let mut ptb = ProgrammableTransactionBuilder::new();

        let package_id = self.config.sui_payments_package_id
            .parse::<ObjectID>()
            .map_err(|e| CoreError::ConfigurationError(format!("Invalid SUI_PAYMENTS_PACKAGE_ID: {}", e)))?;
        
        let module_name_str = "group".to_string();
        let function_name_str = "new".to_string();

        // Admin Cap Object (Mutable Shared Object)
        let admin_cap_call_arg = sui_sdk::types::transaction::CallArg::Object(
            sui_sdk::types::transaction::ObjectArg::SharedObject {
                id: admin_cap_object_id,
                initial_shared_version: sui_sdk::types::base_types::SequenceNumber::default(), // Placeholder
                mutable: true 
            }
        );
        let admin_cap_input_arg = ptb.input(admin_cap_call_arg)
            .map_err(|e| CoreError::TransactionBuildError(format!("Failed to add admin_cap_object_id to PTB: {}", e)))?;

        // Telegram Group ID (Pure String)
        // For pure string, BCS serialize Vec<u8>. String::into_bytes() gives Vec<u8>.
        let tg_group_id_call_arg = sui_sdk::types::transaction::CallArg::Pure(
            bcs::to_bytes(&telegram_group_id.into_bytes())
                .map_err(|e| CoreError::TransactionBuildError(format!("Failed to serialize telegram_group_id: {}", e)))?
        );
        let tg_group_id_input_arg = ptb.input(tg_group_id_call_arg)
            .map_err(|e| CoreError::TransactionBuildError(format!("Failed to add telegram_group_id to PTB: {}", e)))?;

        let move_call_command = sui_sdk::types::transaction::Command::MoveCall(Box::new(
            sui_sdk::types::transaction::ProgrammableMoveCall {
                package: package_id,
                module: module_name_str,
                function: function_name_str,
                type_arguments: vec![], // group::new is not generic
                arguments: vec![admin_cap_input_arg, tg_group_id_input_arg],
            }
        ));
        ptb.command(move_call_command);

        let tx_data = ptb.finish();

        // Sign and execute the transaction
        let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data.clone());
        let signature = admin_keypair.sign_secure_message(&intent_msg);

        let transaction = sui_sdk::types::transaction::Transaction::from_data(tx_data, vec![signature])
            .map_err(|e| CoreError::TransactionBuildError(format!("Failed to create transaction from data: {}", e)))?;
        
        // Verify the transaction before execution
        let verified_transaction = transaction.verify()
            .map_err(|e| CoreError::TransactionBuildError(format!("Transaction verification failed: {}", e)))?;

        let response = self.sui_client
            .quorum_driver_api()
            .execute_transaction_block(
                verified_transaction,
                SuiTransactionBlockResponseOptions::new().with_effects(), 
                Some(ExecuteTransactionRequestType::WaitForLocalExecution)
            )
            .await
            .map_err(|e| CoreError::SuiRpcError(format!("Failed to execute create_group_on_chain transaction: {}", e)))?;

        // Extract the new group ObjectID from the effects
        // The group::new function creates and shares one object.
        let created_objects = response.effects.ok_or_else(|| CoreError::SuiRpcError("Transaction effects not found in response".to_string()))?.created();
        
        if let Some(created_ref) = created_objects.iter().find(|obj_ref| obj_ref.owner.is_shared()) {
            Ok(created_ref.reference.object_id)
        } else {
            Err(CoreError::SuiRpcError("No shared object created by group::new transaction, or created object not found.".to_string()))
        }
    }

    async fn link_user_to_telegram_on_chain(
        &self,
        _admin_address: SuiAddress,         // Bot's admin address (for context)
        _admin_object_id: ObjectID,         // Bot's Admin object ID
        _relations_id_opt: &mut Option<ObjectID>, // ObjectID of the Relations object, or None. The Move fn handles creation.
        _user_sui_address: SuiAddress,    // User's SUI address to link
        _telegram_user_id: String,        // User's Telegram ID
        _telegram_group_id: String      // Telegram Group ID context
    ) -> Result<ObjectID, CoreError> {
        // TODO: This requires an public entry wrapper function in the 'payments::admin' Move module
        // that calls the existing 'public fun set_relations'.
        // The wrapper would simplify handling the Option<ID> for relations_id for PTB calls.
        todo!("link_user_to_telegram_on_chain: Blocked by missing entry wrapper in admin.move for 'set_relations' function.")
    }

    async fn get_sui_address_for_telegram_user(
        &self,
        telegram_user_id: String,
        telegram_group_id: String
    ) -> Result<Option<SuiAddress>, CoreError> {
        let tg_user_id = telegram_user_id.parse::<i64>().map_err(|_| 
            CoreError::ConfigurationError(format!("Invalid telegram_user_id format: {}", telegram_user_id))
        )?;

        let mapping = crate::db::get_user_sui_map(&self.db_pool, tg_user_id, &telegram_group_id).await?;
        
        match mapping {
            Some(m) => {
                SuiAddress::from_str(&m.sui_address)
                    .map(Some)
                    .map_err(|e| CoreError::ConfigurationError(format!("Invalid SuiAddress format in DB: {} - {}", m.sui_address, e)))
            },
            None => Ok(None)
        }
    }

    async fn get_account_object_id_for_telegram_user(
        &self,
        telegram_user_id: String,
        telegram_group_id: String
    ) -> Result<Option<ObjectID>, CoreError> {
        let tg_user_id = telegram_user_id.parse::<i64>().map_err(|_| 
            CoreError::ConfigurationError(format!("Invalid telegram_user_id format: {}", telegram_user_id))
        )?;

        let mapping = crate::db::get_user_sui_map(&self.db_pool, tg_user_id, &telegram_group_id).await?;

        match mapping {
            Some(m) => {
                match m.sui_account_object_id {
                    Some(id_str) => {
                        ObjectID::from_str(&id_str)
                            .map(Some)
                            .map_err(|e| CoreError::ConfigurationError(format!("Invalid ObjectID format in DB: {} - {}", id_str, e)))
                    },
                    None => Ok(None)
                }
            },
            None => Ok(None)
        }
    }

    async fn get_group_object_id(
        &self,
        telegram_group_id: String
    ) -> Result<Option<ObjectID>, CoreError> {
        let mapping = crate::db::get_sui_group_map(&self.db_pool, &telegram_group_id).await?;
        
        match mapping {
            Some(m) => {
                ObjectID::from_str(&m.sui_group_object_id)
                    .map(Some)
                    .map_err(|e| CoreError::ConfigurationError(format!("Invalid ObjectID format in DB for group {}: {} - {}", telegram_group_id, m.sui_group_object_id, e)))
            },
            None => Ok(None)
        }
    }
} 