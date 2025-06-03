module sui_squard::account {
  use std::string::String;
  use sui::{dynamic_field as df, event, coin::{Coin}};
  use sui_squard::admin::Admin;

  const EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION: u64 = 1;
  const ENOT_FOUND_BALANCE: u64 = 2;
  const EMISMATCHED_TELEGRAM_ID: u64 = 3;

  
  public struct Account has key, store {
    id: UID,
    account_id: ID,
    telegram_id: String,
  }

  public struct AccountEvent has copy, drop{
    account_id: ID,
    telegram_id: String,
  }

  public struct AccountBalance<phantom T> has copy, drop, store { }

  public entry fun create_new_account(admin: &Admin, telegram_id: String, ctx: &mut TxContext): ID {
    assert!(admin.get_address() == ctx.sender(), EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION);

    let id = object::new(ctx);
    let account_id = object::uid_to_inner(&id);

    let account = Account {
      id,
      account_id,
      telegram_id,
    };

    event::emit(AccountEvent {
      account_id,
      telegram_id,
    });

    transfer::share_object(account);

    account_id
  }

  public entry fun fund<T>(self: &mut Account, telegram_id: String, coin: Coin<T>) {
    assert!(self.telegram_id == telegram_id, EMISMATCHED_TELEGRAM_ID);

    let account_balance_type = AccountBalance<T> { };

    if(df::exists_(&self.id, account_balance_type)) {
      let balance: &mut Coin<T> = df::borrow_mut(&mut self.id, account_balance_type);
      balance.join(coin);
    } else {
      df::add(&mut self.id, account_balance_type, coin);
    }
  }

  public entry fun withdraw<T>(self: &mut Account, admin: &Admin, amount: u64, recipient: address, ctx: &mut TxContext) {
    assert!(admin.get_address() == ctx.sender(), EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION);

    let account_balance_type = AccountBalance<T> { };
    assert!(df::exists_(&self.id, account_balance_type), ENOT_FOUND_BALANCE);
    let balance: &mut Coin<T> = df::borrow_mut(&mut self.id, account_balance_type);
    balance.split_and_transfer(amount, recipient, ctx);
  }

  public fun payment<T>(self: &mut Account, admin: &Admin, recipient: &mut Account, amount: u64, ctx: &mut TxContext) {
    assert!(admin.get_address() == ctx.sender(), EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION);

    let account_balance_type = AccountBalance<T> { };

    assert!(df::exists_(&self.id, account_balance_type), ENOT_FOUND_BALANCE);

    let balance: &mut Coin<T> = df::borrow_mut(&mut self.id, account_balance_type);

    let coin = balance.split<T>(amount, ctx);

    if(df::exists_(&recipient.id, account_balance_type)) {
      let recipient_balance: &mut Coin<T> = df::borrow_mut(&mut recipient.id, account_balance_type);
      recipient_balance.join(coin);
    } else {
      df::add(&mut recipient.id, account_balance_type, coin);
    };
  }

  public fun get_balance<T>(self: &Account): u64 {
    let account_balance_type = AccountBalance<T> { };
    assert!(df::exists_(&self.id, account_balance_type), ENOT_FOUND_BALANCE);
    let balance: &Coin<T> = df::borrow(&self.id, account_balance_type);
    balance.value()
  }

  #[test_only]
  public fun borrow_mut(self: &mut Account): &mut UID {
    &mut self.id
  }
}