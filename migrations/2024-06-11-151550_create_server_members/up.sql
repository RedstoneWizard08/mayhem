CREATE TABLE IF NOT EXISTS server_members (
    user_id SERIAL NOT NULL REFERENCES users(id),
    server_id SERIAL NOT NULL REFERENCES servers(id),
    nickname TEXT,

    PRIMARY KEY(user_id, server_id)
);
