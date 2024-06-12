// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "channelkind"))]
    pub struct Channelkind;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "theme"))]
    pub struct Theme;
}

diesel::table! {
    channel_categories (id) {
        id -> Int4,
        server_id -> Int4,
        name -> Text,
        created -> Int8,
        updated -> Nullable<Int8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Channelkind;

    channels (id) {
        id -> Int4,
        server_id -> Int4,
        category_id -> Int4,
        name -> Text,
        kind -> Channelkind,
        created -> Int8,
        updated -> Nullable<Int8>,
    }
}

diesel::table! {
    friends (user_id, friend_id) {
        user_id -> Int4,
        friend_id -> Int4,
    }
}

diesel::table! {
    member_roles (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        content -> Text,
        created -> Int8,
        updated -> Nullable<Int8>,
        edited -> Bool,
        to_friend -> Bool,
        sender_id -> Int4,
        channel_id -> Int4,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        server_id -> Int4,
        name -> Text,
        color -> Nullable<Text>,
        created -> Int8,
        updated -> Nullable<Int8>,
        send_messages -> Bool,
        join_voice -> Bool,
        enable_camera -> Bool,
    }
}

diesel::table! {
    server_members (user_id, server_id) {
        user_id -> Int4,
        server_id -> Int4,
        nickname -> Nullable<Text>,
    }
}

diesel::table! {
    server_settings (id) {
        id -> Int4,
        server_id -> Int4,
        created -> Int8,
        updated -> Nullable<Int8>,
        is_public -> Bool,
    }
}

diesel::table! {
    servers (id) {
        id -> Int4,
        name -> Text,
        icon_url -> Nullable<Text>,
        created -> Int8,
        updated -> Nullable<Int8>,
        owner_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Theme;

    user_settings (id) {
        id -> Int4,
        user_id -> Int4,
        created -> Int8,
        updated -> Nullable<Int8>,
        theme -> Theme,
    }
}

diesel::table! {
    user_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        created -> Int8,
        last_used -> Nullable<Int8>,
        value -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password -> Text,
        birthday -> Int8,
        created -> Int8,
        updated -> Nullable<Int8>,
    }
}

diesel::joinable!(channel_categories -> servers (server_id));
diesel::joinable!(channels -> channel_categories (category_id));
diesel::joinable!(channels -> servers (server_id));
diesel::joinable!(member_roles -> roles (role_id));
diesel::joinable!(member_roles -> users (user_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (sender_id));
diesel::joinable!(roles -> servers (server_id));
diesel::joinable!(server_members -> servers (server_id));
diesel::joinable!(server_members -> users (user_id));
diesel::joinable!(server_settings -> servers (server_id));
diesel::joinable!(servers -> users (owner_id));
diesel::joinable!(user_settings -> users (user_id));
diesel::joinable!(user_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    channel_categories,
    channels,
    friends,
    member_roles,
    messages,
    roles,
    server_members,
    server_settings,
    servers,
    user_settings,
    user_tokens,
    users,
);
