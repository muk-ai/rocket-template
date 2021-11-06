table! {
    deleted_users (id) {
        id -> Int4,
        user_id -> Uuid,
        user_firebase_uid -> Varchar,
        user_deleted_at -> Timestamptz,
        user_created_at -> Timestamptz,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        description -> Text,
        completed -> Bool,
        created_at -> Timestamptz,
        user_id -> Uuid,
    }
}

table! {
    users (firebase_uid) {
        firebase_uid -> Varchar,
        created_at -> Timestamptz,
        id -> Uuid,
    }
}

allow_tables_to_appear_in_same_query!(
    deleted_users,
    tasks,
    users,
);
