
#[test_only]
module sui_squard::admin_tests {
    use sui_squard::admin::{Self, Admin};
    use sui::test_scenario::{Self as ts};
    use std::string;

    const EVALUES_DOES_NOT_MATCH: u64 = 1;

    const ADMIN: address = @0x100;
    const USER: address = @0x200;
    const OTHER_USER: address = @0x300;

    #[test]
    fun test_create_admin() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        ts::end(ts);
    }

    #[test]
    fun test_add_relations() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), USER, ts.ctx());
        
        ts.next_tx(ADMIN);

        let telegram_id = admin_obj.borrow_telegram_id(relations_id, USER);

        assert!(telegram_id == string::utf8(b"tg_test"), EVALUES_DOES_NOT_MATCH);

        ts::return_shared(admin_obj);
        
        ts::end(ts);
    }

    #[test, expected_failure(abort_code = sui_squard::admin::ETELEGRAM_DOES_NOT_EXIST)]
    fun test_get_telegram_id_fail() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), USER, ts.ctx());
        
        ts.next_tx(ADMIN);

        admin_obj.borrow_telegram_id(relations_id, OTHER_USER);

        abort 2
    }
}
