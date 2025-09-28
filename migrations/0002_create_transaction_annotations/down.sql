-- Down: Drop trigger, function, and table
DROP TRIGGER IF EXISTS trg_annotations_updated_at ON transaction_annotations;
DROP FUNCTION IF EXISTS update_updated_at_column();
DROP TABLE IF EXISTS transaction_annotations;
