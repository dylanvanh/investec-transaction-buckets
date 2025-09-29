-- Up: Create investec_transactions (PostgreSQL)
CREATE TABLE investec_transactions (
    id SERIAL PRIMARY KEY,
    account_id TEXT NOT NULL,
    tx_type TEXT NOT NULL,
    transaction_type TEXT,
    status TEXT NOT NULL,
    description TEXT NOT NULL,
    card_number TEXT,
    posted_order REAL,
    posting_date TEXT,
    value_date TEXT,
    action_date TEXT,
    transaction_date TEXT,
    amount REAL NOT NULL,
    running_balance REAL,
    uuid TEXT UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
