use super::User;
use crate::connection::DbConn;
use crate::schema::users;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "users"]
struct InsertableUser {
    uid: String,
}

pub fn find(uid: String, conn: &DbConn) -> QueryResult<User> {
    users::table.find(uid).get_result::<User>(&**conn)
}

pub fn insert(uid: String, conn: &DbConn) -> QueryResult<User> {
    let new_user = InsertableUser { uid };
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(&**conn)
}
