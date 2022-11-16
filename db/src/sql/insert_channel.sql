INSERT INTO server_channels
    (name, server, role_permissions, channel_permissions, channel_user_permissions)
    VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (id) DO UPDATE SET
    name = $1,
    server = $2,
    role_permissions = $3,
    channel_permissions = $4,
    channel_user_permissions = $5;
