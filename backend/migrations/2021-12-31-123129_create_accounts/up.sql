-- Bank accounts the user has.
CREATE TABLE bank_accounts (
  id SERIAL PRIMARY KEY,
  alias TEXT NOT NULL,
  iban_number TEXT NOT NULL,
  balance_cents BIGINT NOT NULL --This will overflow when I get rich
);

-- Statement files uploaded by the user.
CREATE TABLE bank_transaction_statements (
  id SERIAL PRIMARY KEY,
  account_id INTEGER NOT NULL REFERENCES bank_accounts(id),
  start_date DATE NOT NULL,
  end_date DATE NOT NULL
);

-- Statement lines.
CREATE TABLE bank_transactions (
  id SERIAL PRIMARY KEY,
  account_id INTEGER REFERENCES bank_accounts(id) NOT NULL,
  statement_id INTEGER REFERENCES bank_transaction_statements(id) NOT NULL,
  date DATE NOT NULL,
  recipient TEXT NOT NULL,
  description TEXT NOT NULL,
  transaction_type TEXT NOT NULL,
  amount_cents BIGINT NOT NULL,
  balance_cents BIGINT NOT NULL
);