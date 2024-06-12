-- This is immutable, so there is no `updated` column.
CREATE TABLE IF NOT EXISTS user_tokens (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users(id),
    created BIGINT NOT NULL,
    last_used BIGINT, -- Unix timestamp of when the token was last used
    value TEXT NOT NULL
);
