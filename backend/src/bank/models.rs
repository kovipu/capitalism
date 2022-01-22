use chrono::NaiveDate;

use crate::schema::{bank_transaction_statements, bank_transactions};

#[derive(Insertable)]
#[table_name = "bank_transaction_statements"]
pub struct BankTransactionStatement {
    pub account_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Insertable)]
#[table_name = "bank_transactions"]
pub struct BankTransaction {
    pub account_id: i32,
    pub statement_id: i32,
    pub date: NaiveDate,
    pub recipient: String,
    pub description: String,
    pub transaction_type: String,
    pub amount_cents: i64,
    pub balance_cents: i64,
}

#[derive(Queryable, Debug)]
pub struct BankAccount {
    pub id: i32,
    pub alias: String,
    pub iban: String,
    pub balance_cents: i64,
}
