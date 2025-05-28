module sui_squard::account {
  use std::string::String;
  use sui::{dynamic_field as df, event, coin::{Coin}};
  use sui_squard::admin::{Admin};

  const EMismatchedSenderAccount: u64 = 1;
  const ENotFoundBalance: u64 = 3;
  
  public struct Account has key, store {
    id: UID,
    account_id: ID,
    telegram_id: String,
    wallet: address,
  }

  public struct AccountEvent has copy, drop{
    account_id: ID,
    wallet: address,
    telegram_id: String,
  }

  public struct AccountBalance<phantom T> has copy, drop, store { }

  public entry fun create_new_account(admin: &mut Admin, relations_id: ID, ctx: &mut TxContext): ID {
    let id = object::new(ctx);
    let account_id = object::uid_to_inner(&id);

    let telegram_id = admin.borrow_telegram_id(relations_id, ctx.sender());

    let account = Account {
      id,
      account_id,
      telegram_id: *telegram_id,
      wallet: ctx.sender()
    };

    event::emit(AccountEvent {
      account_id,
      telegram_id: *telegram_id,
      wallet: ctx.sender(),
    });

    transfer::share_object(account);

    account_id
  }

  public entry fun fund<T>(self: &mut Account, coin: Coin<T>, ctx: &mut TxContext) {
    assert!(self.wallet == ctx.sender(), EMismatchedSenderAccount);

    let account_balance_type = AccountBalance<T> { };

    if(df::exists_(&self.id, account_balance_type)) {
      let balance: &mut Coin<T> = df::borrow_mut(&mut self.id, account_balance_type);
      balance.join(coin);
    } else {
      df::add(&mut self.id, account_balance_type, coin);
    }
  }

  public entry fun withdraw<T>(self: &mut Account, amount: u64, ctx: &mut TxContext) {
    assert!(self.wallet == ctx.sender(), EMismatchedSenderAccount);

    let account_balance_type = AccountBalance<T> { };
    assert!(df::exists_(&self.id, account_balance_type), ENotFoundBalance);
    let balance: &mut Coin<T> = df::borrow_mut(&mut self.id, account_balance_type);
    balance.split_and_transfer(amount, ctx.sender(), ctx);
  }

  public fun payment<T>(self: &mut Account, recipient: &mut Account, amount: u64, ctx: &mut TxContext) {
    assert!(self.wallet == ctx.sender(), EMismatchedSenderAccount);

    let account_balance_type = AccountBalance<T> { };

    assert!(df::exists_(&self.id, account_balance_type), ENotFoundBalance);

    let balance: &mut Coin<T> = df::borrow_mut(&mut self.id, account_balance_type);

    let coin = balance.split<T>(amount, ctx);

    if(df::exists_(&recipient.id, account_balance_type)) {
      let recipient_balance: &mut Coin<T> = df::borrow_mut(&mut recipient.id, account_balance_type);
      recipient_balance.join(coin);
    } else {
      df::add(&mut recipient.id, account_balance_type, coin);
    };
  }

  public fun borrow_account_id(self: &Account): &ID {
    &self.account_id
  }

  public fun get_address(self: &Account): address {
    self.wallet
  }

  public fun get_balance<T>(self: &Account): u64 {
    let account_balance_type = AccountBalance<T> { };
    assert!(df::exists_(&self.id, account_balance_type), ENotFoundBalance);
    let balance: &Coin<T> = df::borrow(&self.id, account_balance_type);
    balance.value()
  }
}