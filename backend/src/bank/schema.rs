use chrono::NaiveDate;
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
struct BankAccount {
    /// Unique id of the account.
    id: String,
    /// Friendly alias of the account.
    alias: String,
    /// The account number.
    iban: String,
    /// Transactions of the account.
    transactions: Vec<BankTransaction>,
}

#[derive(GraphQLObject)]
struct BankTransaction {
    /// Unique id of the transaction.
    id: String,
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
