module sui_squard::admin {
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

  public struct AccountRelation has store {
    account: address,
    telegram_id: String,
  }

  public struct Relations has key, store {
    id: UID,
    relations_id: ID,
    relations: vector<AccountRelation>,
  }

  public struct AdminEvent has copy, drop{
    admin_id: ID,
    wallet: address,
  }

  public struct ADMIN has drop {}

  fun init(otw: ADMIN, ctx: &mut TxContext) {
    package::claim_and_keep(otw, ctx);

    let admin_cap = AdminCap { id: object::new(ctx) };

    transfer::transfer(admin_cap, ctx.sender());
  }

  public fun initialize_admin(admin_cap: AdminCap, ctx: &mut TxContext) {
    let admin = Admin { id: object::new(ctx), account: ctx.sender() };

    let admin_id = object::uid_to_inner(&admin.id);

    let AdminCap { id } = admin_cap;
    object::delete(id);

    event::emit(AdminEvent { admin_id, wallet: ctx.sender() });

    transfer::share_object(admin);
  }

  public fun set_relations(self: &mut Admin, relations_id_opt: &mut Option<ID>, telegram_id: String, user: address, ctx: &mut TxContext): ID {
    assert!(self.account == ctx.sender(), EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION);

    let relations_id;
    if(option::is_some<ID>(relations_id_opt)) {
      relations_id = option::extract(relations_id_opt);
      let relations = dof::borrow_mut<ID, Relations>(self.borrow_mut(), relations_id);

      let relation = AccountRelation { account: user, telegram_id };

      relations.relations.push_back(relation);

    } else {
      let id = object::new(ctx);
      relations_id = object::uid_to_inner(&id);

      let mut relations = Relations { id, relations_id, relations: vector::empty() };
      let relation = AccountRelation { account: user, telegram_id };

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

  public(package) fun borrow_telegram_id(self: &Admin, relations_id: ID, account: address): &String {
    let relations = dof::borrow<ID, Relations>(self.borrow(), relations_id);

    let mut index_opt = relations.relations.find_index!(|r| r.account == account);

    assert!(option::is_some(&index_opt), ETELEGRAM_DOES_NOT_EXIST);

    let index = index_opt.extract();

    let relation = relations.relations.borrow(index);

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