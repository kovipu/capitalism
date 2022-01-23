use diesel::prelude::*;
use diesel::Queryable;
use juniper::{graphql_object, FieldResult};

use crate::graphql_schema::Context;
use crate::schema::bank_accounts;
use crate::schema::bank_accounts::dsl::*;

/*
 * Bank Query
 */
pub struct BankQuery;

#[graphql_object(context = Context)]
impl BankQuery {
    /// Find all bank accounts.
    fn accounts(context: &Context) -> FieldResult<Vec<BankAccount>> {
        let accounts: Vec<BankAccount> = bank_accounts.load(&context.dbpool.get()?)?;

        Ok(accounts)
    }

    /// Find a bank account by id.
    fn account(context: &Context, account_id: i32) -> FieldResult<BankAccount> {
        let account: BankAccount = bank_accounts
            .find(account_id)
            .first(&context.dbpool.get()?)?;

        Ok(account)
    }
}

#[derive(Queryable, Debug)]
pub struct BankAccount {
    pub id: i32,
    pub alias: String,
    pub iban_number: String,
    pub balance_cents: i64,
}

#[graphql_object]
impl BankAccount {
    /// Unique id of the account.
    fn id(&self) -> juniper::ID {
        juniper::ID::from(self.id.to_string())
    }

    /// Friendly alias of the account.
    fn alias(&self) -> &str {
        &self.alias
    }

    /// The account number.
    fn iban_number(&self) -> &str {
        &self.iban_number
    }

    /// Balance of the account in cents.
    fn balance_cents(&self) -> i32 {
        self.balance_cents
            .try_into()
            .expect("Too much money to fit in an i32.")
    }
}

/*
 * Bank Mutation
 */
#[derive(Insertable)]
#[table_name = "bank_accounts"]
pub struct NewBankAccount {
    pub alias: String,
    pub iban_number: String,
    pub balance_cents: i64,
}

pub struct BankMutation;

#[graphql_object(context = Context)]
impl BankMutation {
    /// Create a new account with empty balance.
    fn create_account(
        context: &Context,
        account_alias: String,
        account_iban_number: String,
    ) -> FieldResult<BankAccount> {
        // Create the account
        let account = NewBankAccount {
            alias: account_alias,
            iban_number: account_iban_number,
            balance_cents: 0,
        };

        let inserted_account: BankAccount = diesel::insert_into(bank_accounts)
            .values(&account)
            .get_result(&context.dbpool.get()?)?;

        Ok(inserted_account)
    }
}
