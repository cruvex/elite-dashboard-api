CREATE TABLE IF NOT EXISTS skin_history (
    id          SERIAL PRIMARY KEY,
    uuid        UUID NOT NULL,
    texture_id  TEXT NOT NULL,
    timestamp   TIMESTAMPTZ NOT NULL,
    source      TEXT NOT NULL CHECK (source IN ('task', 'manual'))
);

ALTER TABLE IF EXISTS skin_history OWNER TO postgres;