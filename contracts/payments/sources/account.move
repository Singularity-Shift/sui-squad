module payments::account {
  use std::string::String;
  use sui::{dynamic_field as df, event, coin::{Coin}};
  use payments::admin::{Admin};
  use payments::group::Group;
  use payments::admin;

  const EMismatchedSenderAccount: u64 = 1;
  const EMismatchedGroup: u64 = 2;
  const ENotFoundBalance: u64 = 3;
  
  public struct Account has key, store {
    id: UID,
    account_id: ID,
    group_id: ID,
    telegram_id: String,
    wallet: address,
  }

  public struct AccountEvent has copy, drop{
    account_id: ID,
    group_id: ID,
    wallet: address,
    telegram_id: String,
  }

  public struct AccountBalance<phantom T> has copy, drop, store { }

  public entry fun create_new_account(admin: &mut Admin, relations_id: ID, group: &Group, ctx: &mut TxContext): ID {
    let id = object::new(ctx);
    let account_id = object::uid_to_inner(&id);

    let group_id = group.borrow_group_id();
    let group_telegram_id = group.get_telegram_group_id();

    let telegram_id = admin.borrow_telegram_id(relations_id, ctx.sender(), group_telegram_id);

    let account = Account {
      id,
      account_id,
      group_id: *group_id,
      telegram_id: *telegram_id,
      wallet: ctx.sender()
    };

    event::emit(AccountEvent {
      account_id,
      group_id: *group_id,
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

  public fun payment<T>(self: &mut Account, group: &Group, admin: &Admin, fees_id: ID, recipient: &mut Account, amount: u64, ctx: &mut TxContext) {
    assert!(self.wallet == ctx.sender(), EMismatchedSenderAccount);

    let account_balance_type = AccountBalance<T> { };

    assert!(df::exists_(&self.id, account_balance_type), ENotFoundBalance);

    let balance: &mut Coin<T> = df::borrow_mut(&mut self.id, account_balance_type);

    assert!(group.borrow_group_id() == self.group_id, EMismatchedGroup);

    if(group.get_if_pay_fees()) {
      let fees_amount = (admin::get_fees(admin, fees_id) * amount / 100);

      balance.split_and_transfer(fees_amount, admin::get_address(admin) , ctx)
    };

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