INSERT INTO servers (name, roles, members, channels, owner)
    VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (id) DO UPDATE SET
    name = $1,
    roles = $2,
    members = $3,
    channels = $4,
    owner = $5;
