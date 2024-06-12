CREATE TABLE IF NOT EXISTS messages (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    created BIGINT NOT NULL,
    updated BIGINT,
    edited BOOLEAN NOT NULL DEFAULT false,
    to_friend BOOLEAN NOT NULL DEFAULT false,
    sender_id SERIAL NOT NULL REFERENCES users(id),
    channel_id SERIAL NOT NULL REFERENCES channels(id)
);
