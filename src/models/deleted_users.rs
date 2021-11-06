pub mod repository;

#[derive(Debug, Queryable)]
pub struct DeletedUser {
    id: i32,
    user_id: uuid::Uuid,
    user_firebase_uid: String,
    user_deleted_at: chrono::NaiveDateTime,
    user_created_at: chrono::NaiveDateTime,
}
