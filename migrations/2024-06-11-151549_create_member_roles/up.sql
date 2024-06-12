CREATE TABLE IF NOT EXISTS member_roles (
    user_id SERIAL NOT NULL REFERENCES users(id),
    role_id SERIAL NOT NULL REFERENCES roles(id),
    PRIMARY KEY(user_id, role_id)
);
