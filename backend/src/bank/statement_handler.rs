use std::error::Error;

use actix_multipart::Multipart;
use csv::StringRecord;
use chrono::NaiveDate;
use diesel::prelude::*;
use encoding_rs::ISO_8859_10;
use encoding_rs_io::DecodeReaderBytesBuilder;
use futures_util::stream::StreamExt as _;

use crate::db::PgPool;
use crate::schema::bank_accounts::dsl::*;
use crate::schema::bank_transactions;

use super::models::{BankAccount, BankTransaction};

pub async fn read_statement(
    account_id: i32,
    mut payload: Multipart,
    pool: &PgPool,
) -> Result<bool, Box<dyn Error>> {
    // find if account exists and get previous balance
    let mut balance: i64 = bank_accounts
        .filter(id.eq(account_id))
        .load::<BankAccount>(&pool.get()?)?
        .first()
        .ok_or_else(|| "Account not found".to_string())?
        .balance_cents;

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

    for transaction in transactions.into_iter().rev() {
        let get_field = |field: usize| -> Result<String, Box<dyn Error>> {
            let field = transaction.get(field).ok_or_else(|| {
                format!("Missing field: {}", field)
            })?;
            Ok(field.trim().to_string())
        };

        let date = NaiveDate::parse_from_str(&get_field(0)?, "%d.%m.%Y")?;
        let recipient = get_field(1)?;
        let transaction_type = get_field(2)?;
        let description = get_field(3)?;
        let amount_cents: i64 =
            (get_field(4)?.replace(',', ".").parse::<f64>()? * 100.0) as i64;

        balance = balance + amount_cents;

        let transaction = BankTransaction {
            account_id,
            date,
            recipient,
            transaction_type,
            description,
            amount_cents,
            balance_cents: balance,
        };

        // insert transaction
        diesel::insert_into(bank_transactions::table)
            .values(&transaction)
            .execute(&pool.get()?)?;
    }

    // update account balance
    diesel::update(bank_accounts.filter(id.eq(account_id)))
        .set(balance_cents.eq(balance))
        .execute(&pool.get()?)?;

    Ok(true)
}
