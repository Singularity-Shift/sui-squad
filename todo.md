# Sui Squad Bot - Todo List

## Phase 1: Sui Contract Integration

### 1. Finalize Core Decisions
- [ ] **Transaction Signing Strategy:**
    - [ ] Confirm: Bot uses a configured admin keypair for administrative contract calls (`group::new`, `admin::set_relations`, `admin::set_fees`).
    - [ ] Confirm: User-specific transactions (`account::create_new_account`, `account::fund`, `account::withdraw`, wrapped `account::payment`) require user to sign a Programmable Transaction Block (PTB) provided by the bot (e.g., serialized PTB for external signing).

### 2. `sui-squad-core` Crate Updates
- [ ] **Dependencies:**
    - [x] Add `sui-sdk` to `sui-squad-core/Cargo.toml`. (Partially done, confirm version and features)
- [x] **Configuration (`src/config.rs` & `.env.example`):**
    - [x] Add `sui_rpc_url: String`.
    - [x] Add `sui_payments_package_id: String`.
    - [x] Add `sui_admin_object_id: String` (from `admin::initialize_admin`).
    - [x] Add `sui_fees_id: String` (from `admin::initialize_admin`).
    - [x] Add `sui_relations_id: String` (from `admin::set_relations`, decide if global or per-group).
    - [x] Add `sui_admin_mnemonic_path: Option<String>` (for bot to sign admin transactions).
    - [x] Update `.env.example` with these new variables.
- [ ] **Sui Gateway (`src/sui_gateway.rs`):**
    - [x] Define `SuiGateway` trait:
        - `create_account_on_chain_ptb(user_sui_address: SuiAddress, admin_object_id: ObjectID, relations_id: ObjectID, group_object_id: ObjectID) -> Result<TransactionBlock, CoreError>`
        - `get_account_balance(account_object_id: ObjectID, coin_type_tag: TypeTag) -> Result<u64, CoreError>`
        - `prepare_fund_account_ptb(user_sui_address: SuiAddress, account_object_id: ObjectID, coin_object_ids: Vec<ObjectID>, coin_type_tag: TypeTag) -> Result<TransactionBlock, CoreError>`
        - `prepare_withdraw_from_account_ptb(user_sui_address: SuiAddress, account_object_id: ObjectID, amount: u64, coin_type_tag: TypeTag) -> Result<TransactionBlock, CoreError>`
        - `prepare_payment_ptb(...) -> Result<TransactionBlock, CoreError>` (Requires entry wrapper in contract)
        - `create_group_on_chain(admin_address: SuiAddress, admin_object_id: ObjectID, telegram_group_id: String) -> Result<ObjectID, CoreError>` (Signed by bot)
        - `link_user_to_telegram_on_chain(admin_address: SuiAddress, admin_object_id: ObjectID, relations_id: &mut Option<ObjectID>, user_sui_address: SuiAddress, telegram_user_id: String, telegram_group_id: String) -> Result<ObjectID, CoreError>` (Signed by bot)
        - `get_sui_address_for_telegram_user(telegram_user_id: String, telegram_group_id: String) -> Result<Option<SuiAddress>, CoreError>` (from DB)
        - `get_account_object_id_for_telegram_user(telegram_user_id: String, telegram_group_id: String) -> Result<Option<ObjectID>, CoreError>` (from DB)
        - `get_group_object_id(telegram_group_id: String) -> Result<Option<ObjectID>, CoreError>` (from DB)
    - [ ] Implement `LiveSuiGateway`:
        - [x] `new(config: Config) -> Result<Self, CoreError>` (constructor, added `SuiClientInitializationError`)
        - [ ] `create_account_on_chain_ptb(...) -> Result<ProgrammableTransaction, CoreError>`
            - [x] Use `ProgrammableTransactionBuilder`.
            - [x] Call `payments::account::create_new_account(admin: &mut Admin, relations_id: ID, group: &Group)` (scaffolded call).
            - [x] Map gateway params to Move call args (scaffolded with `CallArg::Object`).
        - [ ] `get_account_balance(account_object_id: ObjectID, coin_type_tag: TypeTag) -> Result<u64, CoreError>`
            - [x] Use `sui_client.coin_read_api().get_balance(address, coin_type)`.
            - [x] **Challenge**: `account_object_id` is an `ObjectID`. `get_balance` needs a `SuiAddress`.
            - [x] How to map `account_object_id` (presumably the ID of our custom `Account` object) to a `SuiAddress` that owns coins?
                - [x] If the `Account` object *is* the coin owner, its ID *is* the `SuiAddress`. (Implemented this assumption)
                - [ ] If the `Account` object *stores* the user's `SuiAddress`, we need to fetch and read the `Account` object first.
                - [x] Clarify: Is the `account_object_id` the address itself, or an object containing the address? (Assumed it's effectively the address)
        - [x] `prepare_fund_account_ptb(...) -> Result<ProgrammableTransaction, CoreError>`
            - [x] Use `ProgrammableTransactionBuilder`.
            - [x] Call `payments::account::deposit(&mut Account, Coin<T>)`.
            - [x] Handle `coin_object_ids` (merge if multiple, use single if one).
            - [x] Map gateway params to Move call args (correctly using `Argument` and `CallArg` types).
        - [x] `prepare_withdraw_from_account_ptb(...) -> Result<ProgrammableTransaction, CoreError>`
            - [x] Use `ProgrammableTransactionBuilder`.
            - [x] Call `payments::account::withdraw(&mut Account, amount: u64)`.
            - [x] Map gateway params to Move call args (account as shared mut obj, amount as pure).
            - [x] Note: Move function transfers coin to `ctx.sender()` directly.
        - [ ] `prepare_payment_ptb(...) -> Result<ProgrammableTransaction, CoreError>`
            - [ ] Use `ProgrammableTransactionBuilder`.
            - [ ] **Blocked**: Requires `entry fun entry_payment<T>(...)` wrapper in `payments::account.move` for the existing `payment` function.
            - [ ] Define parameters based on Move `payment` function: `from_account_id`, `group_id`, `admin_id`, `fees_id`, `to_account_id`, `amount`, `coin_type_tag`.
            - [ ] Map gateway params to Move call args.
        - [ ] `create_group_on_chain(admin_address: SuiAddress, admin_object_id: ObjectID, telegram_group_id: String) -> Result<ObjectID, CoreError>` (Signed by bot)
            - [x] Load admin keypair from `sui_admin_mnemonic_path`.
            - [x] Build PTB for `payments::group::new(admin: &mut Admin, telegram_group_id: String)`.
            - [x] Sign and execute transaction with admin key.
            - [x] Parse new group `ObjectID` from transaction effects.
        - Methods for PTB construction using `TransactionBlockBuilder`.
        - Methods for admin functions using `SuiClient::transaction_builder()` and `SuiClient::signer_and_gas_station()` with bot's admin key.
        - [x] `execute_signed_transaction(signed_tx: SignedTransaction) -> Result<SuiTransactionBlockResponse, CoreError>` (scaffolded)
        - [x] `get_sui_address_for_telegram_user(telegram_user_id: String, telegram_group_id: String) -> Result<Option<SuiAddress>, CoreError>` (from DB)
        - [x] `get_account_object_id_for_telegram_user(telegram_user_id: String, telegram_group_id: String) -> Result<Option<ObjectID>, CoreError>` (from DB)
        - [x] `get_group_object_id(telegram_group_id: String) -> Result<Option<ObjectID>, CoreError>` (from DB)
- [ ] **Database (`src/db/mod.rs` & `src/db/migrations/`):**
    - [x] Define schema for `user_sui_map` table (`telegram_user_id`, `telegram_group_id`, `sui_address`, `sui_account_object_id`).
    - [x] Define schema for `sui_groups_map` table (`telegram_group_id`, `sui_group_object_id`).
    - [x] Create SQLx migration files for these tables.
    - [x] Implement DB functions to store/retrieve these mappings.
    - [x] Update `init_db` to run migrations.
- [ ] **Error Handling (`src/error.rs`):**
    - [x] Add specific error variants for `SuiGateway` operations (e.g., `SuiRpcError`, `TransactionBuildError`, `ObjectNotFound`).

### 3. `sui-squad-bot` Crate Updates
- [ ] **Command Handlers (`src/main.rs` & `src/commands/bot_commands.rs`):**
    - [ ] `/createaccount`:
        - Initial call: Inform user about linking their Sui address via admin.
        - Subsequent call (if address linked):
            - Fetch user's Sui address from DB.
            - Call `sui_gateway.create_account_on_chain_ptb()`.
            - Provide serialized PTB to user for signing.
            - Handle user pasting back signed transaction, then call `sui_gateway.execute_signed_transaction()`.
            - Store `sui_account_object_id` in DB upon success.
    - [ ] `/getbalance [token_symbol]`:
        - Fetch `sui_account_object_id` for user from DB.
        - Determine `coin_type_tag` from `token_symbol` (needs a mapping).
        - Call `sui_gateway.get_account_balance()`.
    - [ ] `/admin_create_group <telegram_group_id>`:
        - Call `sui_gateway.create_group_on_chain()`.
        - Store `sui_group_object_id` in DB, mapping to `telegram_group_id`.
    - [ ] `/admin_link_user <user_sui_address> <telegram_user_id> <telegram_group_id>`:
        - Fetch `relations_id` (global or from group data).
        - Call `sui_gateway.link_user_to_telegram_on_chain()`.
        - Store mapping in `user_sui_map` table.
    - [ ] `/help`: Update with new commands and flow.
- [ ] **Dependency Injection:**
    - [ ] Inject `LiveSuiGateway` and DB connection pool into command handlers.

### 4. Sui Contracts (`contracts/payments/`)
- [ ] **`account.move`:**
    - [ ] Consider adding an `entry fun entry_payment<T>(...)` that wraps the existing public `payment` function, taking necessary Object IDs and signed by the user.

### 5. Documentation & Testing
- [ ] Write/update `README.md` for admin setup (package deployment, initial admin/group creation, linking users).
- [ ] Document the user flow for account creation and transaction signing.
- [ ] Manual testing of all commands and contract interactions on a testnet.

## Future Enhancements (Post Phase 1)
- [ ] Support for various SUI token types (fungible tokens).
- [ ] More complex contract interactions (e.g., `/createpool`, `/claim` if those are still planned).
- [ ] Event listeners for on-chain events to update bot state.
- [ ] Improved UX for transaction signing (e.g., zkLogin, sponsored transactions if contracts are adapted). 