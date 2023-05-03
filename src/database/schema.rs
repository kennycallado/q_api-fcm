// @generated automatically by Diesel CLI.

diesel::table! {
    fcm_messages (id) {
        id -> Int4,
        title -> Varchar,
        body -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    fcm_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    fcm_messages,
    fcm_tokens,
);
