CREATE TABLE IF NOT EXISTS server_settings (
    id SERIAL PRIMARY KEY,
    server_id SERIAL NOT NULL REFERENCES servers(id),
    created BIGINT NOT NULL,
    updated BIGINT,

    -- Settings
    is_public BOOLEAN NOT NULL DEFAULT false
);
