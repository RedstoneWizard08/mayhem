CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    servers INTEGER[] NOT NULL,
    settings INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS user_settings (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS servers (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    roles INTEGER[] NOT NULL,
    members INTEGER[] NOT NULL,
    channels INTEGER[] NOT NULL,
    owner INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS server_roles (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    members INTEGER[] NOT NULL,

    -- Format: "(permission_id):(access_type)"
    -- Example: "0:0" = "SendMessage:GRANTED"
    permissions TEXT[] NOT NULL
);

CREATE TABLE IF NOT EXISTS server_members (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    nick TEXT NOT NULL,
    roles INTEGER[] NOT NULL
);

CREATE TABLE IF NOT EXISTS server_channels (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    server INTEGER NOT NULL,

    -- Format: "(role_id):(access_type)"
    -- Example: "0:0" = "SomeRole:GRANTED"
    role_permissions TEXT[] NOT NULL,

    -- Format: "(permission_id):(role_id):(access_type)"
    -- Example: "0:0:0" = "SendMessage:SomeRole:GRANTED"
    channel_permissions TEXT[] NOT NULL,

    -- Format: "(permission_id):(user_id):(access_type)"
    -- Example: "0:0:0" = "SendMessage:SomeUser:GRANTED"
    channel_user_permissions TEXT[] NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
    id SERIAL PRIMARY KEY,
    server INTEGER NOT NULL,
    channel INTEGER NOT NULL,
    sender INTEGER NOT NULL,
    sender_name TEXT NOT NULL,
    sender_image TEXT NOT NULL,
    content TEXT NOT NULL,
    date_sent TEXT NOT NULL
);
