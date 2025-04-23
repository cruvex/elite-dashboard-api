CREATE TABLE IF NOT EXISTS tracked_uuids_history (
    id         SERIAL PRIMARY KEY,
    uuid       UUID NOT NULL,
    operation  TEXT NOT NULL CHECK (operation IN ('add', 'remove')),
    timestamp  TIMESTAMPTZ NOT NULL
);

ALTER TABLE IF EXISTS tracked_uuids_history OWNER TO postgres;