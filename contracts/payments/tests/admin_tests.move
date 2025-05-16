
#[test_only]
module payments::admin_tests {
    use payments::admin::{Self, Admin};
    use sui::test_scenario::{Self as ts};
    use std::string;

    const EVALUES_DOES_NOT_MATCH: u64 = 1;

    const ADMIN: address = @0x100;
    const USER: address = @0x200;

    #[test]
    fun test_create_admin() {
        let mut ts = ts::begin(ADMIN);

        let fees_id = admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        admin_obj.set_fees(fees_id, 1, ts.ctx());

        ts.next_tx(ADMIN);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test]
    fun test_add_relations() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), string::utf8(b"tg_group_test"), USER, ts.ctx());
        
        ts.next_tx(ADMIN);

        let telegram_id = admin_obj.borrow_telegram_id(relations_id, USER, string::utf8(b"tg_group_test"));

        assert!(telegram_id == string::utf8(b"tg_test"), EVALUES_DOES_NOT_MATCH);

        ts::return_shared(admin_obj);
        
        ts::end(ts);
    }

    #[test, expected_failure(abort_code = payments::admin::ETELEGRAM_DOES_NOT_EXIST)]
    fun test_get_telegram_id_fail() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), string::utf8(b"tg_group_test"), USER, ts.ctx());
        
        ts.next_tx(ADMIN);

        admin_obj.borrow_telegram_id(relations_id, USER, string::utf8(b"tg_group_test_2"));

        abort 2
    }
}
