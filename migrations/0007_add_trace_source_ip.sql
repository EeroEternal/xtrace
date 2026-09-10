CREATE OR REPLACE FUNCTION xtrace_normalize_source_ip(raw text)
RETURNS text
LANGUAGE sql
IMMUTABLE
AS $$
  SELECT CASE
    WHEN raw IS NULL THEN ''
    WHEN btrim(raw) = '' THEN ''
    ELSE (
      WITH cleaned AS (
        SELECT CASE
          WHEN btrim(raw) ~ '^\[.*\](:[0-9]+)?$' THEN
            substring(btrim(raw) from '^\[([^\]]+)\]')
          WHEN btrim(raw) ~ '^[0-9]{1,3}(\.[0-9]{1,3}){3}:[0-9]+$' THEN
            split_part(btrim(raw), ':', 1)
          ELSE btrim(raw)
        END AS host
      )
      SELECT CASE
        WHEN lower(host) LIKE '::ffff:%' THEN substring(lower(host) from 8)
        ELSE lower(host)
      END
      FROM cleaned
    )
  END
$$;

CREATE OR REPLACE FUNCTION xtrace_extract_source_ip(meta jsonb)
RETURNS text
LANGUAGE sql
IMMUTABLE
AS $$
  SELECT xtrace_normalize_source_ip(
    COALESCE(
      NULLIF(btrim(COALESCE(meta->>'sourceIp', '')), ''),
      NULLIF(btrim(COALESCE(meta->>'source_ip', '')), ''),
      NULLIF(btrim(COALESCE(meta->>'clientIp', '')), ''),
      NULLIF(btrim(COALESCE(meta->>'client_ip', '')), ''),
      ''
    )
  )
$$;

ALTER TABLE traces
  ADD COLUMN IF NOT EXISTS source_ip TEXT
  GENERATED ALWAYS AS (xtrace_extract_source_ip(metadata)) STORED;

CREATE INDEX IF NOT EXISTS idx_traces_project_source_ip_timestamp
  ON traces (project_id, source_ip, timestamp DESC);
