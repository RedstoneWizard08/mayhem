CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL, -- This is an argon2id-hashed string that is the user's password.
    birthday BIGINT NOT NULL, -- This is a unix timestamp representing the user's date. The time will always be 00:00:00.
    created BIGINT NOT NULL,
    updated BIGINT
);
