CREATE TABLE bank_accounts (
  id SERIAL PRIMARY KEY,
  alias TEXT NOT NULL,
  ìban TEXT NOT NULL,
  balance_cents BIGINT NOT NULL --This will overflow when I get rich
);

CREATE TABLE bank_transactions (
  id SERIAL PRIMARY KEY,
  account_id INTEGER REFERENCES bank_accounts(id) NOT NULL,
  date DATE NOT NULL,
  recipient TEXT NOT NULL,
  description TEXT NOT NULL,
  transaction_type TEXT NOT NULL,
  amount_cents BIGINT NOT NULL,
  balance_cents BIGINT NOT NULL
);