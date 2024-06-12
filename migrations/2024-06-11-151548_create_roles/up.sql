CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    server_id SERIAL NOT NULL REFERENCES servers(id),
    name TEXT NOT NULL,
    color TEXT,
    created BIGINT NOT NULL,
    updated BIGINT,

    -- Role Permissions
    send_messages BOOLEAN NOT NULL DEFAULT true,
    join_voice BOOLEAN NOT NULL DEFAULT true,
    enable_camera BOOLEAN NOT NULL DEFAULT true
);
