INSERT INTO users (first_name, last_name, email, username, password, servers, settings)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT (id) DO UPDATE SET
    first_name = $1,
    last_name = $2,
    email = $3,
    username = $4,
    password = $5,
    servers = $6,
    settings = $7;
