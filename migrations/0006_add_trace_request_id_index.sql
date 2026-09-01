CREATE INDEX IF NOT EXISTS idx_traces_metadata_request_id
ON traces ((metadata->>'request_id'))
WHERE metadata ? 'request_id';
