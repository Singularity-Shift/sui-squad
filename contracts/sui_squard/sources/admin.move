module sui_squard::admin {
  use std::string::String;
  use sui::package;
  use sui::event;

  const EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION: u64 = 1;
  const ETELEGRAM_DOES_NOT_EXIST: u64 = 2;

  public struct AdminCap has key {
    id: UID,
  }

  public struct Admin has key {
    id: UID,
    account: address,
  }

  public struct AccountRelation has store {
    account: address,
    telegram_id: String,
  }

  public struct Relation has key, store {
    id: UID,
    users: vector<AccountRelation>,
  }

  public struct AdminEvent has copy, drop{
    admin_id: ID,
    wallet: address,
  }

  public struct RelationEvent has copy, drop {
    relation_id: ID,
    users: vector<String>,
  }

  public struct ADMIN has drop {}

  fun init(otw: ADMIN, ctx: &mut TxContext) {
    package::claim_and_keep(otw, ctx);

    let admin_cap = AdminCap { id: object::new(ctx) };

    transfer::transfer(admin_cap, ctx.sender());
  }

  public fun initialize_admin(admin_cap: AdminCap, ctx: &mut TxContext) {
    let admin = Admin { id: object::new(ctx), account: ctx.sender() };
    let relation = Relation { id: object::new(ctx), users: vector::empty() };

    let admin_id = object::uid_to_inner(&admin.id);
    
    let relation_id = object::uid_to_inner(&relation.id);

    let AdminCap { id } = admin_cap;
    object::delete(id);

    event::emit(AdminEvent { admin_id, wallet: ctx.sender() });

    event::emit(RelationEvent { relation_id, users: vector::empty() });

    transfer::transfer(admin, ctx.sender());
    transfer::share_object(relation);
  }

  public fun set_relations(self: &Admin, relation: &mut Relation, telegram_id: String, user: address, ctx: &mut TxContext) {
    assert!(self.account == ctx.sender(), EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION);
 
    let some_relation = relation.users.find_index!(|r| r.account == user);

    if(option::is_none(&some_relation)) {

      let relations_id = object::uid_to_inner(&relation.id);
      let account_relation = AccountRelation { account: user, telegram_id };

      relation.users.push_back(account_relation);

      let relations_vector = relation.users.map_ref!(|r| r.telegram_id);
      event::emit(RelationEvent { relation_id: relations_id, users: relations_vector });
    }
  }

  public(package) fun borrow_mut(self: &mut Admin): &mut UID {
    &mut self.id
  }

  public(package) fun borrow(self: &Admin): &UID {
    &self.id
  }

  public(package) fun borrow_telegram_id(relation: &Relation, account: address): &String {
    let mut index_opt = relation.users.find_index!(|r| r.account == account);

    assert!(option::is_some(&index_opt), ETELEGRAM_DOES_NOT_EXIST);

    let index = index_opt.extract();

    let relation = relation.users.borrow(index);

    &relation.telegram_id
  }

  public(package) fun get_address(self: &Admin): address {
    self.account
  }

  #[test_only]
  public fun init_test(ctx: &mut TxContext) {
    let admin_cap = AdminCap { id: object::new(ctx) };

    init(ADMIN {}, ctx);

    admin_cap.initialize_admin(ctx);
  }
}