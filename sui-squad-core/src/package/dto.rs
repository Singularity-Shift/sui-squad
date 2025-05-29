use core::fmt;
use std::env;

pub enum AdminFunction {
    SetRelations,
}

pub enum AccountFunction {
    CreateNewAccount,
    Fund,
    Withdraw,
    Payment,
    BorrowAccountId,
    GetAddress,
    GetBalance,
}

pub enum Event {
    AdminEvent,
    AccountEvent,
    RelationEvent,
}


pub enum Function {
    Admin(AdminFunction),
    Account(AccountFunction),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let package = env::var("SUI_SQUARD_PACKAGE_ID").expect("SUI_SQUARD_PACKAGE_ID is not set");
        match self {
            Event::AdminEvent => write!(f, "{}::admin::AdminEvent", package),
            Event::AccountEvent => write!(f, "{}::account::AccountEvent", package),
            Event::RelationEvent => write!(f, "{}::admin::RelationEvent", package),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let package = env::var("SUI_SQUARD_PACKAGE_ID").expect("SUI_SQUARD_PACKAGE_ID is not set");
        match self {
            Function::Admin(AdminFunction::SetRelations) => write!(f, "{}::admin::set_relations", package),
            Function::Account(AccountFunction::CreateNewAccount) => write!(f, "{}::account::create_new_account", package),
            Function::Account(AccountFunction::Fund) => write!(f, "{}::account::fund", package),
            Function::Account(AccountFunction::Withdraw) => write!(f, "{}::account::withdraw", package),
            Function::Account(AccountFunction::Payment) => write!(f, "{}::account::payment", package),
            Function::Account(AccountFunction::BorrowAccountId) => write!(f, "{}::account::borrow_account_id", package),
            Function::Account(AccountFunction::GetAddress) => write!(f, "{}::account::get_address", package),
            Function::Account(AccountFunction::GetBalance) => write!(f, "{}::account::get_balance", package),
        }
    }
}


