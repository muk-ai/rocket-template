use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::schema::tasks;
use crate::task::Task;

#[get("/tasks/<id>")]
pub fn tasks_get(id: i32, conn: DbConn) -> Result<Json<Task>, Status> {
    let query_result: QueryResult<Task> = tasks::table.find(id).get_result::<Task>(&*conn);
    query_result
        .map(|task| Json(task))
        .map_err(|error| match error {
            Error::NotFound => Status::NotFound,
            _ => Status::InternalServerError,
        })
}
