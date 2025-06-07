
#[test_only]
module sui_squad::admin_tests {
    use sui_squad::admin;
    use sui::test_scenario::{Self as ts};

    const ADMIN: address = @0x100;

    #[test]
    fun test_create_admin() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        ts::end(ts);
    }
}
