INSERT INTO server_members (name, nick, roles)
    VALUES ($1, $2, $3)
ON CONFLICT (id) DO UPDATE SET
    name = $1,
    nick = $2,
    roles = $3;
