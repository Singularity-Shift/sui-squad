module payments::admin {
  use std::string::String;
  use sui::package;
  use sui::{dynamic_object_field as dof, event};

  const EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION: u64 = 1;
  const ETELEGRAM_DOES_NOT_EXIST: u64 = 2;

  public struct AdminCap has key {
    id: UID,
  }

  public struct Admin has key {
    id: UID,
    account: address,
  }

  public struct Fees has key, store {
    id: UID,
    fees_id: ID,
    amount: u64,
  }

  public struct AccountRelation has store {
    account: address,
    telegram_id: String,
    group_telegram_id: String,
  }

  public struct Relations has key, store {
    id: UID,
    relations_id: ID,
    relations: vector<AccountRelation>,
  }

  public struct FeesEvent has copy, drop {
    fees_id: ID,
    amount: u64
  }

  public struct ADMIN has drop {}

  fun init(otw: ADMIN, ctx: &mut TxContext) {
    package::claim_and_keep(otw, ctx);

    let admin_cap = AdminCap { id: object::new(ctx) };

    transfer::transfer(admin_cap, ctx.sender());
  }

  public fun initialize_admin(admin_cap: AdminCap, amount: u64, ctx: &mut TxContext): ID {
    let mut admin = Admin { id: object::new(ctx), account: ctx.sender() };

    let id = object::new(ctx);

    let fees_id = object::uid_to_inner(&id);

    let fees = Fees { id, fees_id, amount };

    dof::add(&mut admin.id, fees_id, fees);

    let AdminCap { id } = admin_cap;
    object::delete(id);

    transfer::share_object(admin);
    
    event::emit(FeesEvent { fees_id , amount });

    fees_id
  }

  public fun set_fees(self: &mut Admin, fees_id: ID, amount: u64, ctx: &mut TxContext) {
    assert!(self.account == ctx.sender(), EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION);

    let fees = dof::borrow_mut<ID, Fees>(self.borrow_mut(), fees_id);

    fees.amount = amount;

    event::emit(FeesEvent { fees_id , amount });
  }

  public fun set_relations(self: &mut Admin, relations_id_opt: &mut Option<ID>, telegram_id: String, group_telegram_id: String, user: address, ctx: &mut TxContext): ID {
    assert!(self.account == ctx.sender(), EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION);

    let relations_id;
    if(option::is_some<ID>(relations_id_opt)) {
      relations_id = option::extract(relations_id_opt);
      let relations = dof::borrow_mut<ID, Relations>(self.borrow_mut(), relations_id);

      let relation = AccountRelation { account: user, telegram_id, group_telegram_id };

      relations.relations.push_back(relation);

    } else {
      let id = object::new(ctx);
      relations_id = object::uid_to_inner(&id);

      let mut relations = Relations { id, relations_id, relations: vector::empty() };
      let relation = AccountRelation { account: user, telegram_id, group_telegram_id };

      relations.relations.push_back(relation);

      dof::add(&mut self.id, relations_id, relations);
    };

    relations_id
  }

  public(package) fun borrow_mut(self: &mut Admin): &mut UID {
    &mut self.id
  }

  public(package) fun borrow(self: &Admin): &UID {
    &self.id
  }

  public(package) fun borrow_telegram_id(self: &Admin, relations_id: ID, account: address, group_telegram_id: String): &String {
    let relations = dof::borrow<ID, Relations>(self.borrow(), relations_id);

    let mut index_opt = relations.relations.find_index!(|r| r.account == account && r.group_telegram_id == group_telegram_id);

    assert!(option::is_some(&index_opt), ETELEGRAM_DOES_NOT_EXIST);

    let index = index_opt.extract();

    let relation = relations.relations.borrow(index);

    &relation.telegram_id
  }

  public(package) fun get_address(self: &Admin): address {
    self.account
  }

  public fun get_fees(self: &Admin, fees_id: ID): u64 {
    let fees = dof::borrow<ID, Fees>(self.borrow(), fees_id);

    fees.amount
  }

  #[test_only]
  public fun init_test(ctx: &mut TxContext): ID {
    let admin_cap = AdminCap { id: object::new(ctx) };

    init(ADMIN {}, ctx);

    let fees_id = admin_cap.initialize_admin(1, ctx);

    fees_id
  }
}