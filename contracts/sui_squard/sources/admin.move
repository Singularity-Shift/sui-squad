module sui_squad::admin {
  use sui::package;
  use sui::event;

  public struct AdminCap has key {
    id: UID,
  }

  public struct Admin has key {
    id: UID,
    account: address,
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

  public(package) fun borrow_mut(self: &mut Admin): &mut UID {
    &mut self.id
  }

  public(package) fun borrow(self: &Admin): &UID {
    &self.id
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