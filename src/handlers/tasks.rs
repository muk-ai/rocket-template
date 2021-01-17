use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Deserialize;

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

#[derive(Insertable)]
#[table_name = "tasks"]
struct InsertableTask {
    description: String,
    completed: bool,
}

impl InsertableTask {
    fn from_task(task: TaskDescriptionData) -> InsertableTask {
        InsertableTask {
            description: task.description,
            completed: false,
        }
    }
}

#[derive(Deserialize)]
pub struct TaskDescriptionData {
    description: String,
}

#[post("/tasks", format = "application/json", data = "<task>")]
pub fn tasks_post(task: Json<TaskDescriptionData>, conn: DbConn) -> Result<Status, Status> {
    let query_result = diesel::insert_into(tasks::table)
        .values(&InsertableTask::from_task(task.into_inner()))
        .get_result::<Task>(&*conn);
    query_result
        .map(|_task| Status::Created)
        .map_err(|_error| Status::InternalServerError)
}
