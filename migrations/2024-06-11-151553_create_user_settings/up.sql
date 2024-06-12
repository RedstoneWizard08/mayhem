CREATE TYPE Theme AS ENUM ('dark', 'light');

CREATE TABLE IF NOT EXISTS user_settings (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users(id),
    created BIGINT NOT NULL,
    updated BIGINT,

    -- Settings
    theme Theme NOT NULL DEFAULT 'dark'
);
