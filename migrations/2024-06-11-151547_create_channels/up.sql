CREATE TYPE ChannelKind AS ENUM ('text', 'voice', 'announcement', 'event');

CREATE TABLE IF NOT EXISTS channels (
    id SERIAL PRIMARY KEY,
    server_id SERIAL NOT NULL REFERENCES servers(id),
    category_id SERIAL NOT NULL REFERENCES channel_categories(id),
    name TEXT NOT NULL,
    kind ChannelKind NOT NULL DEFAULT 'text',
    created BIGINT NOT NULL,
    updated BIGINT
);
