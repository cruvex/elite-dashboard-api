CREATE TABLE IF NOT EXISTS name_history (
    id         SERIAL PRIMARY KEY,
    uuid       UUID NOT NULL,
    ign        TEXT NOT NULL,
    timestamp  TIMESTAMPTZ NOT NULL,
    source     TEXT NOT NULL CHECK (source IN ('task', 'manual'))
);

ALTER TABLE IF EXISTS name_history OWNER TO postgres;