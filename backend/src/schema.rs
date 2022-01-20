table! {
    bank_accounts (id) {
        id -> Int4,
        alias -> Text,
        ìban -> Text,
        balance_cents -> Int8,
    }
}

table! {
    bank_transactions (id) {
        id -> Int4,
        account_id -> Int4,
        date -> Date,
        recipient -> Text,
        description -> Text,
        transaction_type -> Text,
        amount_cents -> Int8,
        balance_cents -> Int8,
    }
}

joinable!(bank_transactions -> bank_accounts (account_id));

allow_tables_to_appear_in_same_query!(
    bank_accounts,
    bank_transactions,
);
