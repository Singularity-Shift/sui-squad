
#[test_only]
module sui_squard::account_tests {
    use sui_squard::admin::{Self, Admin};
    use sui_squard::account::{Self, Account};
    use sui::test_scenario::{Self as ts, Scenario};
    use sui::coin::{Self, Coin};
    use sui::sui::SUI;
    use sui::{dynamic_field as df};
    use std::string;

    const EVALUES_DOES_NOT_MATCH: u64 = 1;

    const ADMIN: address = @0x100;
    const USER: address = @0x200;
    const RECIPIENT: address = @0x300;

    public struct AccountBalance<phantom T> has copy, drop, store { }

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
        
        let account_obj = ts.take_shared_by_id<Account>(account_id);

        ts::return_shared(account_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    } 

    #[test]
    fun test_fund_account() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_shared<Admin>();

        ts.next_tx(ADMIN);

        let account_id = account::create_new_account(&admin_obj, string::utf8(b"tg_test"), ts.ctx());

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        let coin = test_coin(&mut ts, 1000);

        let account_balance_type = AccountBalance<SUI> { };

        let account = account_obj.borrow_mut();

        df::add(account, account_balance_type, coin);

        ts.next_tx(USER);

        let balance: &Coin<SUI> = df::borrow(account, account_balance_type);

        assert!(        balance.value() == 1000, EVALUES_DOES_NOT_MATCH);

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

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        let coin = test_coin(&mut ts, 1000);

        account_obj.fund(string::utf8(b"test_tg"), coin);

        ts.next_tx(ADMIN);

        account_obj.withdraw<SUI>(&admin_obj ,500, USER, ts.ctx());

        assert!(account_obj.get_balance<SUI>() == 500, EVALUES_DOES_NOT_MATCH);

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

        ts.next_tx(USER);

        account_obj.fund(string::utf8(b"test_tg"), test_coin(&mut ts, 100));

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

        let account_id = account::create_new_account(&admin_obj, string::utf8(b"test_tg"), ts.ctx());

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        ts.next_tx(ADMIN);

        let recipient_account_id = account::create_new_account(&admin_obj, string::utf8(b"test_tg"), ts.ctx());

        ts.next_tx(ADMIN);

        account_obj.fund(string::utf8(b"test_tg"), test_coin(&mut ts, 100));

        ts.next_tx(USER);
        
        let mut recipient_account_obj = ts.take_shared_by_id<Account>(recipient_account_id);

        ts.next_tx(ADMIN);

        account_obj.payment<SUI>(&admin_obj, &mut recipient_account_obj, 50, ts.ctx());

        ts.next_tx(ADMIN);

        account_obj.payment<SUI>(&admin_obj, &mut recipient_account_obj, 50, ts.ctx());

        ts.next_tx(RECIPIENT);

        let recipient_balance = recipient_account_obj.get_balance<SUI>();

        assert!(recipient_balance == 100, EVALUES_DOES_NOT_MATCH);

        ts::return_shared(recipient_account_obj);

        ts::return_shared(account_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test, expected_failure(abort_code = sui_squard::account::EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION)]
    fun test_create_account_with_invalid_admin() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_shared<Admin>();

        ts.next_tx(USER);

        account::create_new_account(&admin_obj, string::utf8(b"test_tg"), ts.ctx());

        abort 1
    }

    #[test, expected_failure(abort_code = sui_squard::account::EONLY_AUTHORIZED_ACCOUNTS_CAN_EXECUTE_THIS_OPERATION)]
    fun test_withdraw_funds_with_invalid_admin() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_shared<Admin>();

        ts.next_tx(ADMIN);

        let account_id = account::create_new_account(&admin_obj, string::utf8(b"test_tg"), ts.ctx());

        ts.next_tx(USER);

        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        account_obj.fund(string::utf8(b"test_tg"), test_coin(&mut ts, 100));

        ts.next_tx(USER);

        account_obj.withdraw<SUI>(&admin_obj, 50, USER, ts.ctx());

        abort 1
    }

    #[test, expected_failure(abort_code = sui_squard::account::EMISMATCHED_TELEGRAM_ID)]
    fun test_fund_account_with_invalid_telegram_id() {
        let mut ts = ts::begin(ADMIN);

        admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let admin_obj = ts.take_shared<Admin>();

        ts.next_tx(ADMIN);

        let account_id = account::create_new_account(&admin_obj, string::utf8(b"test_tg"), ts.ctx());

        ts.next_tx(USER);

        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        account_obj.fund(string::utf8(b"test_tg_2"), test_coin(&mut ts, 100));

        abort 2
    }
}
