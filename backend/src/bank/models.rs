use chrono::NaiveDate;

use crate::schema::bank_transactions;

#[derive(Insertable)]
#[table_name = "bank_transactions"]
pub struct BankTransaction {
    pub account_id: i32,
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
