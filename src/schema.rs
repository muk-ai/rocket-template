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
    users (uid) {
        uid -> Varchar,
        created_at -> Timestamptz,
        id -> Uuid,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
