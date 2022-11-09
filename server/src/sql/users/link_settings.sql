INSERT INTO user_settings(user_id)
VALUES ($1)
RETURNING *;
