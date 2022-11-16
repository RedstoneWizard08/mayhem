INSERT INTO user_settings (user_id)
    VALUES ($1)
ON CONFLICT (id) DO UPDATE SET
    user_id = $1;
