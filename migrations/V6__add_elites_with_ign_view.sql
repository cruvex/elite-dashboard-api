CREATE VIEW elites_with_ign AS
SELECT e.*, nh.ign
FROM elites e
LEFT JOIN LATERAL (
    SELECT ign
    FROM name_history
    WHERE uuid = e.minecraft_uuid
    ORDER BY timestamp DESC
    LIMIT 1
) nh ON true;

ALTER VIEW IF EXISTS elites_with_ign OWNER TO postgres;