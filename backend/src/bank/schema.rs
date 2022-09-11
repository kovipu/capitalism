use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::Queryable;
use juniper::{graphql_object, FieldResult};

use crate::graphql_schema::Context;
use crate::schema;
use crate::schema::{bank_accounts, bank_accounts::dsl::*};
use crate::schema::{bank_transactions::dsl::*};
use crate::schema::{bank_transaction_statements::dsl::*};

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
    fn account(context: &Context, accid: i32) -> FieldResult<BankAccount> {
        let account: BankAccount = bank_accounts
            .find(accid)
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

#[graphql_object(context = Context)]
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

    /// Find all statements for this account.
    fn statements(&self, context: &Context) -> FieldResult<Vec<BankTransactionStatement>> {
        let statements: Vec<BankTransactionStatement> = bank_transaction_statements
            .filter(account_id.eq(self.id))
            .load(&context.dbpool.get()?)?;

        Ok(statements)
    }

    /// Find all transactions for this account.
    fn transactions(&self, context: &Context) -> FieldResult<Vec<BankTransaction>> {
        let transactions: Vec<BankTransaction> = bank_transactions
            .filter(account_id.eq(self.id))
            .load(&context.dbpool.get()?)?;

        Ok(transactions)
    }
}

#[derive(Queryable)]
pub struct BankTransaction {
    pub id: i32,
    pub account_id: i32,
    pub statement_id: i32,
    pub date: NaiveDate,
    pub recipient: String,
    pub description: String,
    pub transaction_type: String,
    pub amount_cents: i64,
    pub balance_cents: i64,
}

#[graphql_object]
impl BankTransaction {
    /// Unique id of the transaction.
    fn id(&self) -> juniper::ID {
        juniper::ID::from(self.id.to_string())
    }

    /// The account id of the transaction.
    fn account_id(&self) -> i32 {
        self.account_id
    }

    /// The statement id of the transaction.
    fn statement_id(&self) -> i32 {
        self.statement_id
    }

    /// The date of the transaction.
    fn date(&self) -> NaiveDate {
        self.date
    }

    /// The recipient of the transaction.
    fn recipient(&self) -> &str {
        &self.recipient
    }

    /// The description of the transaction.
    fn description(&self) -> &str {
        &self.description
    }

    /// The type of the transaction.
    fn transaction_type(&self) -> &str {
        &self.transaction_type
    }

    /// The amount of the transaction in cents.
    fn amount_cents(&self) -> i32 {
        self.amount_cents
            .try_into()
            .expect("Too much money to fit in an i32.")
    }

    /// The balance of the account in cents.
    fn balance_cents(&self) -> i32 {
        self.balance_cents
            .try_into()
            .expect("Too much money to fit in an i32.")
    }
}

#[derive(Queryable)]
pub struct BankTransactionStatement {
    id: i32,
    account_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

#[graphql_object(context = Context)]
impl BankTransactionStatement {
    /// Unique id of the statement.
    fn id(&self) -> juniper::ID {
        juniper::ID::from(self.id.to_string())
    }

    /// The account id of the statement.
    fn account_id(&self) -> i32 {
        self.account_id
    }

    /// The start date of the statement.
    fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    /// The end date of the statement.
    fn end_date(&self) -> NaiveDate {
        self.end_date
    }

    /// Find all transactions for this statement.
    fn transactions(&self, context: &Context) -> FieldResult<Vec<BankTransaction>> {
        let transactions: Vec<BankTransaction> = bank_transactions
            .filter(statement_id.eq(self.id))
            .load(&context.dbpool.get()?)?;

        Ok(transactions)
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
