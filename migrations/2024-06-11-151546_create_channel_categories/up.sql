CREATE TABLE IF NOT EXISTS channel_categories (
    id SERIAL PRIMARY KEY,
    server_id SERIAL NOT NULL REFERENCES servers(id),
    name TEXT NOT NULL,
    created BIGINT NOT NULL,
    updated BIGINT
);
