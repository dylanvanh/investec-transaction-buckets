-- Up: Create transaction_annotations and trigger (PostgreSQL)
CREATE TABLE transaction_annotations (
    id SERIAL PRIMARY KEY,
    investec_transaction_id INTEGER NOT NULL UNIQUE,
    bucket TEXT,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (investec_transaction_id) REFERENCES investec_transactions(id) ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER trg_annotations_updated_at
BEFORE UPDATE ON transaction_annotations
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
