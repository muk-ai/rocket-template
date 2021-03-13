use super::User;
use crate::connection::DbConn;
use crate::schema::users;
use diesel::prelude::*;

pub fn find(uid: String, conn: DbConn) -> QueryResult<User> {
    users::table.find(uid).get_result::<User>(&*conn)
}
