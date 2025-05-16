module payments::group {
  use std::string::String;
  use sui::event;
  use payments::admin::Admin;

  public struct Group has key, store {
    id: UID,
    group_id: ID,
    telegram_group_id: String,
    pay_fees: bool,
  }

  public struct GroupEvent has copy, drop {
    group_id: ID,
    telegram_group_id: String,
    pay_fees: bool,
  }

  public entry fun new(admin: &mut Admin, telegram_group_id: String, ctx: &mut TxContext): ID {
    let id = object::new(ctx);
    let group_id = object::uid_to_inner(&id);

    let pay_fees = admin.get_address() != ctx.sender();

    let group = Group {
      id,
      group_id,
      telegram_group_id,
      pay_fees,
    };

    transfer::share_object(group);

    event::emit(GroupEvent {
      group_id,
      telegram_group_id,
      pay_fees,
    });

    group_id
  }

  public(package) fun borrow_group_id(self: &Group): &ID {
    &self.group_id
  }

  public fun get_telegram_group_id(self: &Group): String {
    self.telegram_group_id
  }

  public fun get_if_pay_fees(self: &Group): bool {
    self.pay_fees
  }
}