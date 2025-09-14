CREATE TABLE IF NOT EXISTS transaction_annotations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    investec_transaction_id INTEGER NOT NULL UNIQUE,
    bucket TEXT,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (investec_transaction_id) REFERENCES investec_transactions(id) ON DELETE CASCADE
);

CREATE TRIGGER IF NOT EXISTS trg_annotations_updated_at
AFTER UPDATE ON transaction_annotations
FOR EACH ROW BEGIN
    UPDATE transaction_annotations SET updated_at = datetime('now') WHERE id = OLD.id;
END;
