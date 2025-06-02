
#[test_only]
module sui_squard::account_tests {
    use sui_squard::admin::{Self, Admin};
    use sui_squard::account::{Self, Account};
    use sui::test_scenario::{Self as ts, Scenario};
    use sui::coin::{Self, Coin};
    use sui::sui::SUI;
    use std::string;

    const EVALUES_DOES_NOT_MATCH: u64 = 1;

    const ADMIN: address = @0x100;
    const USER: address = @0x200;
    const RECIPIENT: address = @0x300;

  fun test_coin(ts: &mut Scenario, amount: u64): Coin<SUI> {
    coin::mint_for_testing<SUI>(amount, ts.ctx())
  }

    #[test]
    fun test_create_account() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_shared<Admin>();

        let account_id = account::create_new_account(&admin_obj, string::utf8(b"tg_test"), ts.ctx());

        ts.next_tx(USER);
        
        let _account_obj = ts.take_shared_by_id<Account>(account_id);

        ts::end(ts);
    } 

    #[test]
    fun test_fund_account() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_shared<Admin>();

        ts.next_tx(ADMIN);

        ts.next_tx(ADMIN);


        ts.next_tx(USER);

        let account_id = account::create_new_account(&admin_obj, string::utf8(b"tg_test"), ts.ctx());

        ts.next_tx(USER);
        
        let account_obj = ts.take_shared_by_id<Account>(account_id);

        ts.next_tx(USER);

        let account_balance = account_obj.get_balance<SUI>();

        assert!(account_balance == 1000, EVALUES_DOES_NOT_MATCH);

        ts::return_shared(account_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test]
    fun test_withdraw_funds() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_shared<Admin>();

        ts.next_tx(ADMIN);

        let account_id = account::create_new_account(&admin_obj, string::utf8(b"test_tg"), ts.ctx());

        ts.next_tx(ADMIN);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        account_obj.withdraw<SUI>(&admin_obj ,500, USER, ts.ctx());

        let account_balance = account_obj.get_balance<SUI>();

        assert!(account_balance == 500, EVALUES_DOES_NOT_MATCH);

        ts::return_shared(account_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test]
    fun test_pay_account() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_shared<Admin>();


        ts.next_tx(ADMIN);

        let account_id = account::create_new_account(&admin_obj, string::utf8(b"test_tg"), ts.ctx());

        ts.next_tx(ADMIN);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        ts.next_tx(ADMIN);

        let recipient_account_id = account::create_new_account(&admin_obj, string::utf8(b"test_2") , ts.ctx());

        ts.next_tx(RECIPIENT);
        
        let mut recipient_account_obj = ts.take_shared_by_id<Account>(recipient_account_id);

        ts.next_tx(ADMIN);

        account_obj.payment<SUI>(&admin_obj, &mut recipient_account_obj, 50, ts.ctx());

        ts.next_tx(RECIPIENT);

        let recipient_balance = recipient_account_obj.get_balance<SUI>();

        assert!(recipient_balance == 50, EVALUES_DOES_NOT_MATCH);

        ts::return_shared(recipient_account_obj);

        ts::return_shared(account_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test]
    fun test_pay_account_two_times() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_shared<Admin>();


        ts.next_tx(ADMIN);



        ts.next_tx(USER);

        let account_id = account::create_new_account(&admin_obj, string::utf8(b"test_tg"), ts.ctx());

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        ts.next_tx(ADMIN);

        let recipient_account_id = account::create_new_account(&admin_obj, string::utf8(b"test_tg"), ts.ctx());

        ts.next_tx(ADMIN);
        
        let mut recipient_account_obj = ts.take_shared_by_id<Account>(recipient_account_id);

        ts.next_tx(ADMIN);

        account_obj.payment<SUI>(&admin_obj, &mut recipient_account_obj, 50, ts.ctx());

        ts.next_tx(USER);

        account_obj.payment<SUI>(&admin_obj, &mut recipient_account_obj, 50, ts.ctx());

        ts.next_tx(RECIPIENT);

        let recipient_balance = recipient_account_obj.get_balance<SUI>();

        assert!(recipient_balance == 100, EVALUES_DOES_NOT_MATCH);

        ts::return_shared(recipient_account_obj);

        ts::return_shared(account_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }
}
