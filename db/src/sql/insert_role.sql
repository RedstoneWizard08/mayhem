INSERT INTO server_roles
    (name, members, permissions)
    VALUES ($1, $2, $3)
ON CONFLICT (id) DO UPDATE SET
    name = $1,
    members = $2,
    permissions = $3;
