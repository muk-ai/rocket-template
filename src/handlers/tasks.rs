use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Deserialize;

use crate::connection::DbConn;
use crate::log::{write_log, TraceContext};
use crate::models::tasks::{InsertableTask, Task};
use crate::models::users::User;
use crate::schema::tasks;

#[get("/tasks")]
pub fn tasks_index(
    user: User,
    conn: DbConn,
    trace: Option<&TraceContext>,
) -> Result<Json<Vec<Task>>, Status> {
    write_log("/tasks called", trace);
    let query_result: QueryResult<Vec<Task>> = Task::belonging_to(&user).load::<Task>(&*conn);
    query_result
        .map(|task| Json(task))
        .map_err(|_error| Status::InternalServerError)
}

#[get("/tasks/<id>")]
pub fn tasks_get(user: User, id: i32, conn: DbConn) -> Result<Json<Task>, Status> {
    let query_result: QueryResult<Task> = tasks::table.find(id).get_result::<Task>(&*conn);
    if let Err(error) = query_result {
        return match error {
            Error::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        };
    }
    let task = query_result.unwrap();
    if task.user_id == user.id {
        Ok(Json(task))
    } else {
        Err(Status::Forbidden)
    }
}

#[derive(Deserialize)]
pub struct TaskDescriptionData {
    description: String,
}

#[post("/tasks", format = "application/json", data = "<task>")]
pub fn tasks_post(
    user: User,
    task: Json<TaskDescriptionData>,
    conn: DbConn,
) -> Result<Status, Status> {
    let query_result = diesel::insert_into(tasks::table)
        .values(&InsertableTask::build(
            task.into_inner().description,
            user.id,
        ))
        .get_result::<Task>(&*conn);
    query_result
        .map(|_task| Status::Created)
        .map_err(|_error| Status::InternalServerError)
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "tasks"]
pub struct TaskChangeset {
    completed: Option<bool>,
    description: Option<String>,
}

#[patch("/tasks/<id>", format = "application/json", data = "<task>")]
pub fn tasks_update(
    user: User,
    id: i32,
    task: Json<TaskChangeset>,
    conn: DbConn,
) -> Result<Json<Task>, Status> {
    let query_result: QueryResult<Task> = tasks::table.find(id).get_result::<Task>(&*conn);
    if let Err(error) = query_result {
        return match error {
            Error::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        };
    }
    let found_task = query_result.unwrap();
    if found_task.user_id != user.id {
        return Err(Status::Forbidden);
    }

    let query_result = diesel::update(tasks::table.find(id))
        .set(task.into_inner())
        .get_result(&*conn);
    query_result
        .map(|task| Json(task))
        .map_err(|error| match error {
            Error::NotFound => Status::NotFound,
            _ => Status::InternalServerError,
        })
}

#[delete("/tasks/<id>")]
pub fn tasks_delete(user: User, id: i32, conn: DbConn) -> Result<Status, Status> {
    let query_result: QueryResult<Task> = tasks::table.find(id).get_result::<Task>(&*conn);
    if let Err(error) = query_result {
        return match error {
            Error::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        };
    }
    let task = query_result.unwrap();
    if task.user_id != user.id {
        return Err(Status::Forbidden);
    }

    diesel::delete(tasks::table.find(id))
        .execute(&*conn)
        .map(|_| Status::NoContent)
        .map_err(|_| Status::InternalServerError)
}
