table! {
    tasks (id) {
        id -> Int4,
        description -> Text,
        completed -> Bool,
    }
}

table! {
    users (uid) {
        uid -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
