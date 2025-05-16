
#[test_only]
module payments::group_tests {
    use payments::admin::{Self, Admin};
    use payments::group::{Self, Group};
    use sui::test_scenario::{Self as ts};
    use std::string;

    const EVALUES_DOES_NOT_MATCH: u64 = 1;
    const ADMIN_GROUP_SHOULD_NOT_PAY_FEES: u64 = 2;
    const MOD_SHOULD_PAY_FEES: u64 = 3;

    const ADMIN: address = @0x100;
    const MOD: address = @0x200;

    #[test]
    fun test_new_group() {
        let mut ts = ts::begin(ADMIN);

        let fees_id = admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        admin_obj.set_fees(fees_id, 1, ts.ctx());

        ts.next_tx(ADMIN);

        let group_id = group::new(&mut admin_obj, string::utf8(b"tg_group_test"), ts.ctx());

        ts.next_tx(ADMIN);

        let group_obj = ts.take_shared_by_id<Group>(group_id);

        let group_telegram_id = group_obj.get_telegram_group_id();

        assert!(group_telegram_id == string::utf8(b"tg_group_test"), EVALUES_DOES_NOT_MATCH);

        let group_pay_fees = group_obj.get_if_pay_fees();

        assert!(group_pay_fees == false, ADMIN_GROUP_SHOULD_NOT_PAY_FEES);

        ts::return_shared(group_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test]
    fun test_group_created_for_not_admin() {
        let mut admin = ts::begin(ADMIN);

        let fees_id = admin::init_test( admin.ctx());

        admin.next_tx(ADMIN);

        let mut admin_obj = admin.take_shared<Admin>();

        admin_obj.set_fees(fees_id, 1, admin.ctx());

        admin.next_tx(ADMIN);

        ts::end(admin);

        let mut mod = ts::begin(MOD);

        let group_id = group::new(&mut admin_obj, string::utf8(b"tg_group_test"), mod.ctx());

        mod.next_tx(ADMIN);

        let group_obj = mod.take_shared_by_id<Group>(group_id);

        let group_telegram_id = group_obj.get_telegram_group_id();

        assert!(group_telegram_id == string::utf8(b"tg_group_test"), EVALUES_DOES_NOT_MATCH);

        let group_pay_fees = group_obj.get_if_pay_fees();

        assert!(group_pay_fees == true, MOD_SHOULD_PAY_FEES);

        ts::return_shared(group_obj);

        ts::return_shared(admin_obj);

        ts::end(mod);
    }
      
}
