
#[test_only]
module sui_squard::admin_tests {
    use sui_squard::admin::{Self, Admin};
    use sui::test_scenario::{Self as ts};
    use std::string;
    use sui_squard::admin::Relation;

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

        let admin_obj = ts.take_from_sender<Admin>();

        let mut relation_obj = ts.take_shared<Relation>();

        admin_obj.set_relations(&mut relation_obj, string::utf8(b"tg_test"), USER, ts.ctx());
        
        ts.next_tx(ADMIN);

        let telegram_id = relation_obj.borrow_telegram_id(USER);

        assert!(telegram_id == string::utf8(b"tg_test"), EVALUES_DOES_NOT_MATCH);

        ts::return_to_address(ADMIN, admin_obj);
        ts::return_shared(relation_obj);
        
        ts::end(ts);
    }

    #[test, expected_failure(abort_code = sui_squard::admin::ETELEGRAM_DOES_NOT_EXIST)]
    fun test_get_telegram_id_fail() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_from_sender<Admin>();

        let mut relation_obj = ts.take_shared<Relation>();

        admin_obj.set_relations(&mut relation_obj, string::utf8(b"tg_test"), USER, ts.ctx());
        
        ts.next_tx(ADMIN);

        relation_obj.borrow_telegram_id(OTHER_USER);

        abort 2
    }
}
