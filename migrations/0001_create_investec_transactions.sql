CREATE TABLE IF NOT EXISTS investec_transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

