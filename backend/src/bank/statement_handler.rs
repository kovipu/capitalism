use std::error::Error;

use actix_multipart::Multipart;
use chrono::NaiveDate;
use csv::StringRecord;
use diesel::prelude::*;
use encoding_rs::ISO_8859_10;
use encoding_rs_io::DecodeReaderBytesBuilder;
use futures_util::stream::StreamExt as _;

use crate::db::PgPool;
use crate::schema::bank_accounts::dsl::*;
use crate::schema::{bank_transaction_statements, bank_transactions};

use super::models::{BankAccount, BankTransaction, BankTransactionStatement};

pub async fn read_statement(
    account_id: i32,
    payload: Multipart,
    pool: &PgPool,
) -> Result<(), Box<dyn Error>> {
    // find if account exists and get previous balance
    let mut balance = find_balance(account_id, pool)?;

    // We have to collect the statements in a vector to read them in reverse order.
    let transactions = read_payload(payload).await?;

    let statement_id = insert_statement(&transactions, account_id, pool)?;

    insert_transactions(transactions, &mut balance, account_id, statement_id, pool)?;

    update_balance(account_id, balance, pool)?;

    Ok(())
}

fn find_balance(account_id: i32, pool: &PgPool) -> Result<i64, Box<dyn Error>> {
    let balance: i64 = bank_accounts
        .filter(id.eq(account_id))
        .load::<BankAccount>(&pool.get()?)?
        .first()
        .ok_or_else(|| "Account not found".to_string())?
        .balance_cents;

    Ok(balance)
}

async fn read_payload(mut payload: Multipart) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let mut transactions: Vec<StringRecord> = vec![];

    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data: Vec<u8> = chunk?.iter().cloned().collect();

            // transcode from ISO-8859-10 to UTF-8
            let transcoded_reader = DecodeReaderBytesBuilder::new()
                .encoding(Some(ISO_8859_10))
                .build(data.as_slice());

            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(b';')
                .from_reader(transcoded_reader);

            for result in rdr.records() {
                let result = result?;
                transactions.push(result);
            }
        }
    }

    Ok(transactions)
}

// Insert statement into database.
fn insert_statement(
    transactions: &[StringRecord],
    account_id: i32,
    pool: &PgPool,
) -> Result<i32, Box<dyn Error>> {
    let start_date = transactions
        .last()
        .ok_or_else(|| "No transactions found".to_string())?
        .get(0)
        .ok_or_else(|| "No transactions found".to_string())?;
    let start_date = NaiveDate::parse_from_str(start_date, "%d.%m.%Y")?;

    let end_date = transactions
        .first()
        .ok_or_else(|| "No transactions found".to_string())?
        .get(0)
        .ok_or_else(|| "No transactions found".to_string())?;
    let end_date = NaiveDate::parse_from_str(end_date, "%d.%m.%Y")?;

    let statement = BankTransactionStatement {
        account_id,
        start_date,
        end_date,
    };

    let statement_id: i32 = *diesel::insert_into(bank_transaction_statements::table)
        .values(&statement)
        .returning(bank_transaction_statements::id)
        .get_results(&pool.get()?)?
        .first()
        .ok_or_else(|| "Could not insert statement".to_string())?;

    Ok(statement_id)
}

fn insert_transactions(
    transactions: Vec<StringRecord>,
    balance: &mut i64,
    account_id: i32,
    statement_id: i32,
    pool: &PgPool,
) -> Result<(), Box<dyn Error>> {
    // Format transactions into BankTransaction structs
    let mut formatted_transactions: Vec<BankTransaction> = vec![];

    for transaction in transactions.into_iter().rev() {
        let get_field = |field: usize| -> Result<String, Box<dyn Error>> {
            let field = transaction
                .get(field)
                .ok_or_else(|| format!("Missing field: {}", field))?;
            Ok(field.trim().to_string())
        };

        let date = NaiveDate::parse_from_str(&get_field(0)?, "%d.%m.%Y")?;
        let recipient = get_field(1)?;
        let transaction_type = get_field(2)?;
        let description = get_field(3)?;
        let amount_cents: i64 = (get_field(4)?.replace(',', ".").parse::<f64>()? * 100.0) as i64;

        *balance += amount_cents;

        let transaction = BankTransaction {
            account_id,
            statement_id,
            date,
            recipient,
            transaction_type,
            description,
            amount_cents,
            balance_cents: *balance,
        };

        formatted_transactions.push(transaction);
    }

    diesel::insert_into(bank_transactions::table)
        .values(&formatted_transactions)
        .execute(&pool.get()?)?;

    Ok(())
}

fn update_balance(account_id: i32, balance: i64, pool: &PgPool) -> Result<(), Box<dyn Error>> {
    diesel::update(bank_accounts.filter(id.eq(account_id)))
        .set(balance_cents.eq(balance))
        .execute(&pool.get()?)?;

    Ok(())
}
