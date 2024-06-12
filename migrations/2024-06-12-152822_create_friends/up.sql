CREATE TABLE IF NOT EXISTS friends (
    user_id SERIAL NOT NULL REFERENCES users(id),
    friend_id SERIAL NOT NULL REFERENCES users(id),

    PRIMARY KEY(user_id, friend_id)
);
