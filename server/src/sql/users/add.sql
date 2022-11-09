INSERT INTO users(first_name, last_name, email, password, username)
VALUES ($1, $2, $3, $4, $5)
RETURNING *;
