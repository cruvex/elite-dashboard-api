CREATE TABLE IF NOT EXISTS elites (
    id               SERIAL PRIMARY KEY,
    minecraft_uuid   UUID NOT NULL,
    discord_user_id  TEXT NOT NULL,
    status           TEXT NOT NULL CHECK (status IN ('staff', 'veteran', 'elite', 'trial', 'none')),
    country_code     TEXT NOT NULL,
    birthday         DATE
);

ALTER TABLE IF EXISTS elites OWNER TO postgres;