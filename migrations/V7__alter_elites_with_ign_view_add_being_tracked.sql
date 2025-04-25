-- Add being_tracked column to elites_with_ign view
CREATE OR REPLACE VIEW elites_with_ign AS
SELECT
    e.*,
    nh.ign,
    (tu.uuid IS NOT NULL) AS being_tracked
FROM
    elites e
LEFT JOIN LATERAL (
    SELECT ign
    FROM name_history
    WHERE uuid = e.minecraft_uuid
    ORDER BY timestamp DESC
    LIMIT 1
) nh ON true
LEFT JOIN
    tracked_uuids tu
    ON e.minecraft_uuid = tu.uuid
ORDER BY e.id;

ALTER VIEW IF EXISTS elites_with_ign OWNER TO postgres;