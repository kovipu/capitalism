use chrono::NaiveDate;
use juniper::{graphql_object, GraphQLObject};

pub struct BankQuery;

#[graphql_object]
impl BankQuery {
    fn accounts() -> Vec<BankAccount> {
        // TODO: implement
        vec![BankAccount {
            id: 1,
            alias: "".to_string(),
            iban: "".to_string(),
            balance_cents: 1,
            transactions: vec![],
        }]
    }
}

#[derive(GraphQLObject)]
struct BankAccount {
    /// Unique id of the account.
    id: i32,
    /// Friendly alias of the account.
    alias: String,
    /// The account number.
    iban: String,
    /// Balance of the account in cents.
    balance_cents: i32,
    /// Transactions of the account.
    transactions: Vec<BankTransaction>,
}

// A single statement of transactions.
#[derive(GraphQLObject)]
struct BankTransactionStatement {
    /// Unique id of the statement.
    id: i32,
    /// The account the statement belongs to.
    account_id: i32,
    /// The start date of the statement.
    start_date: NaiveDate,
    /// The end date of the statement.
    end_date: NaiveDate,
    /// Transactions of the statement.
    transactions: Vec<BankTransaction>,
}

// A single transaction to/from the account.
#[derive(GraphQLObject)]
struct BankTransaction {
    /// Unique id of the transaction.
    id: i32,
    /// The date of the transaction.
    date: NaiveDate,
    /// The description of the transaction.
    description: String,
    /// The recipient or sender of the transaction.
    recipient: String,
    /// The type of the transaction.
    transaction_type: String,
    /// The amount of the transaction.
    amount_cents: i32,
    /// Balance of the account after the transaction.
    balance_cents: i32,
}
