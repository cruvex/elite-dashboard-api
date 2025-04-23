CREATE TABLE IF NOT EXISTS tracked_uuids (
    id            SERIAL PRIMARY KEY,
    uuid          UUID NOT NULL,
    last_checked  TIMESTAMPTZ NOT NULL
);

ALTER TABLE IF EXISTS tracked_uuids OWNER TO postgres;