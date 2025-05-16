
#[test_only]
module payments::account_tests {
    use payments::admin::{Self, Admin};
    use payments::group::{Self, Group};
    use payments::account::{Self, Account};
    use sui::test_scenario::{Self as ts, Scenario};
    use sui::coin::{Self, Coin};
    use sui::sui::SUI;
    use std::string;

    const EVALUES_DOES_NOT_MATCH: u64 = 1;

    const ADMIN: address = @0x100;
    const USER: address = @0x200;
    const RECIPIENT: address = @0x300;
    const FAKE_USER: address = @0x400;

  fun test_coin(ts: &mut Scenario, amount: u64): Coin<SUI> {
    coin::mint_for_testing<SUI>(amount, ts.ctx())
  }

    #[test]
    fun test_create_account() {
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

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), group_telegram_id, USER, ts.ctx());

        ts.next_tx(USER);

        let account_id = account::create_new_account(&mut admin_obj, relations_id, &group_obj, ts.ctx());

        ts.next_tx(USER);
        
        let account_obj = ts.take_shared_by_id<Account>(account_id);

        let tg_account = account_obj.get_address();

        assert!(tg_account == ts.ctx().sender(), EVALUES_DOES_NOT_MATCH);

        ts::return_shared(account_obj);

        ts::return_shared(group_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    } 

    #[test]
    fun test_fund_account() {
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

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), group_telegram_id, USER, ts.ctx());

        ts.next_tx(USER);

        let coin = test_coin(&mut ts, 1000);

        let account_id = account::create_new_account(&mut admin_obj, relations_id, &group_obj, ts.ctx());

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        account_obj.fund<SUI>(coin, ts.ctx());

        ts.next_tx(USER);

        let account_balance = account_obj.get_balance<SUI>();

        assert!(account_balance == 1000, EVALUES_DOES_NOT_MATCH);

        ts::return_shared(account_obj);

        ts::return_shared(group_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test]
    fun test_withdraw_funds() {
      let mut ts = ts::begin(ADMIN);

        let fees_id = admin::init_test(ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        admin_obj.set_fees(fees_id, 1, ts.ctx());

        ts.next_tx(ADMIN);

        let group_id = group::new(&mut admin_obj, string::utf8(b"tg_group_test"), ts.ctx());

        ts.next_tx(ADMIN);

        let group_obj = ts.take_shared_by_id<Group>(group_id);

        ts.next_tx(ADMIN);

        let group_telegram_id = group_obj.get_telegram_group_id();

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), group_telegram_id, USER, ts.ctx());

        ts.next_tx(USER);

        let coin = test_coin(&mut ts, 1000);

        let account_id = account::create_new_account(&mut admin_obj, relations_id, &group_obj, ts.ctx());

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        account_obj.fund<SUI>(coin, ts.ctx());

        account_obj.withdraw<SUI>(500, ts.ctx());

        let account_balance = account_obj.get_balance<SUI>();

        assert!(account_balance == 500, EVALUES_DOES_NOT_MATCH);

        ts::return_shared(account_obj);

        ts::return_shared(group_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test]
    fun test_pay_account() {
        let mut ts = ts::begin(ADMIN);

        let fees_id = admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        admin_obj.set_fees(fees_id, 1, ts.ctx());

        let group_id = group::new(&mut admin_obj, string::utf8(b"tg_group_test"), ts.ctx());

        ts.next_tx(ADMIN);

        let group_obj = ts.take_shared_by_id<Group>(group_id);

        let group_telegram_id = group_obj.get_telegram_group_id();

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), group_telegram_id, USER, ts.ctx());

        ts.next_tx(USER);

        let coin = test_coin(&mut ts, 1000);

        let account_id = account::create_new_account(&mut admin_obj, relations_id, &group_obj, ts.ctx());

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        account_obj.fund<SUI>(coin, ts.ctx());

        ts.next_tx(ADMIN);

        let relations_recipient_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test_recipient"), group_telegram_id, RECIPIENT, ts.ctx());

        ts.next_tx(RECIPIENT);

        let recipient_account_id = account::create_new_account(&mut admin_obj, relations_recipient_id, &group_obj, ts.ctx());

        ts.next_tx(RECIPIENT);
        
        let mut recipient_account_obj = ts.take_shared_by_id<Account>(recipient_account_id);

        ts.next_tx(USER);

        account_obj.payment<SUI>(&group_obj, &admin_obj, fees_id, &mut recipient_account_obj, 50, ts.ctx());

        ts.next_tx(RECIPIENT);

        let recipient_balance = recipient_account_obj.get_balance<SUI>();

        assert!(recipient_balance == 50, EVALUES_DOES_NOT_MATCH);

        ts::return_shared(recipient_account_obj);

        ts::return_shared(account_obj);

        ts::return_shared(group_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test]
    fun test_pay_account_two_times() {
        let mut ts = ts::begin(ADMIN);

        let fees_id = admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        admin_obj.set_fees(fees_id, 1, ts.ctx());

        let group_id = group::new(&mut admin_obj, string::utf8(b"tg_group_test"), ts.ctx());

        ts.next_tx(ADMIN);

        let group_obj = ts.take_shared_by_id<Group>(group_id);

        let group_telegram_id = group_obj.get_telegram_group_id();

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), group_telegram_id, USER, ts.ctx());

        ts.next_tx(USER);

        let coin = test_coin(&mut ts, 1000);

        let account_id = account::create_new_account(&mut admin_obj, relations_id, &group_obj, ts.ctx());

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        account_obj.fund<SUI>(coin, ts.ctx());

        ts.next_tx(ADMIN);

        let relations_recipient_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test_recipient"), group_telegram_id, RECIPIENT, ts.ctx());

        ts.next_tx(RECIPIENT);

        let recipient_account_id = account::create_new_account(&mut admin_obj, relations_recipient_id, &group_obj, ts.ctx());

        ts.next_tx(RECIPIENT);
        
        let mut recipient_account_obj = ts.take_shared_by_id<Account>(recipient_account_id);

        ts.next_tx(USER);

        account_obj.payment<SUI>(&group_obj, &admin_obj, fees_id, &mut recipient_account_obj, 50, ts.ctx());

        ts.next_tx(USER);

        account_obj.payment<SUI>(&group_obj, &admin_obj, fees_id, &mut recipient_account_obj, 50, ts.ctx());

        ts.next_tx(RECIPIENT);

        let recipient_balance = recipient_account_obj.get_balance<SUI>();

        assert!(recipient_balance == 100, EVALUES_DOES_NOT_MATCH);

        ts::return_shared(recipient_account_obj);

        ts::return_shared(account_obj);

        ts::return_shared(group_obj);

        ts::return_shared(admin_obj);

        ts::end(ts);
    }

    #[test, expected_failure(abort_code = payments::admin::ETELEGRAM_DOES_NOT_EXIST)]
    fun test_create_account_with_not_existing_tg() {
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

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), group_telegram_id, USER, ts.ctx());

        ts.next_tx(FAKE_USER);  

        account::create_new_account(&mut admin_obj, relations_id, &group_obj, ts.ctx());

        abort 2
    }

    #[test, expected_failure(abort_code = payments::account::EMismatchedSenderAccount)]
    fun pay_with_not_existing_account() {
        let mut ts = ts::begin(ADMIN);

        let fees_id = admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        admin_obj.set_fees(fees_id, 1, ts.ctx());

        let group_id = group::new(&mut admin_obj, string::utf8(b"tg_group_test"), ts.ctx());

        ts.next_tx(ADMIN);

        let group_obj = ts.take_shared_by_id<Group>(group_id);

        let group_telegram_id = group_obj.get_telegram_group_id();

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), group_telegram_id, USER, ts.ctx());

        ts.next_tx(USER);

        let coin = test_coin(&mut ts, 1000);

        let account_id = account::create_new_account(&mut admin_obj, relations_id, &group_obj, ts.ctx());

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        account_obj.fund<SUI>(coin, ts.ctx());

        ts.next_tx(ADMIN);

        let relations_recipient_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test_recipient"), group_telegram_id, RECIPIENT, ts.ctx());

        ts.next_tx(RECIPIENT);

        let recipient_account_id = account::create_new_account(&mut admin_obj, relations_recipient_id, &group_obj, ts.ctx());

        ts.next_tx(RECIPIENT);
        
        let mut recipient_account_obj = ts.take_shared_by_id<Account>(recipient_account_id);

        ts.next_tx(FAKE_USER);

        account_obj.payment<SUI>(&group_obj, &admin_obj, fees_id, &mut recipient_account_obj, 50, ts.ctx());

        abort 1
    }

    #[test, expected_failure(abort_code = payments::account::ENotFoundBalance)]
    fun pay_with_not_existing_object() {
        let mut ts = ts::begin(ADMIN);

        let fees_id = admin::init_test( ts.ctx());

        ts.next_tx(ADMIN);

        let mut admin_obj = ts.take_shared<Admin>();

        admin_obj.set_fees(fees_id, 1, ts.ctx());

        let group_id = group::new(&mut admin_obj, string::utf8(b"tg_group_test"), ts.ctx());

        ts.next_tx(ADMIN);

        let group_obj = ts.take_shared_by_id<Group>(group_id);

        let group_telegram_id = group_obj.get_telegram_group_id();

        let relations_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test"), group_telegram_id, USER, ts.ctx());

        ts.next_tx(USER);

        let account_id = account::create_new_account(&mut admin_obj, relations_id, &group_obj, ts.ctx());

        ts.next_tx(USER);
        
        let mut account_obj = ts.take_shared_by_id<Account>(account_id);

        ts.next_tx(ADMIN);

        let relations_recipient_id = admin_obj.set_relations(&mut option::none(), string::utf8(b"tg_test_recipient"), group_telegram_id, RECIPIENT, ts.ctx());

        ts.next_tx(RECIPIENT);

        let recipient_account_id = account::create_new_account(&mut admin_obj, relations_recipient_id, &group_obj, ts.ctx());

        ts.next_tx(RECIPIENT);
        
        let mut recipient_account_obj = ts.take_shared_by_id<Account>(recipient_account_id);

        ts.next_tx(USER);

        account_obj.payment<SUI>(&group_obj, &admin_obj, fees_id, &mut recipient_account_obj, 50, ts.ctx());

        abort 3
    }
}
