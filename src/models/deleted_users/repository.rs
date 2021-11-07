use crate::connection::DbConn;
use crate::models::deleted_users::DeletedUser;
use crate::models::users::User;
use crate::schema::deleted_users;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "deleted_users"]
struct InsertableDeletedUser {
    user_id: uuid::Uuid,
    user_firebase_uid: String,
    user_created_at: chrono::NaiveDateTime,
}

pub fn insert(user: &User, conn: &DbConn) -> QueryResult<DeletedUser> {
    let record = InsertableDeletedUser {
        user_id: user.id,
        user_firebase_uid: user.firebase_uid.clone(),
        user_created_at: user.created_at,
    };
    diesel::insert_into(deleted_users::table)
        .values(record)
        .get_result(&**conn)
}
