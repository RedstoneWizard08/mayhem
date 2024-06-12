CREATE TABLE IF NOT EXISTS servers (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    icon_url TEXT,
    created BIGINT NOT NULL,
    updated BIGINT,
    owner_id SERIAL NOT NULL REFERENCES users(id)
);
