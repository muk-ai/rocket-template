use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Deserialize;

use crate::connection::DbConn;
use crate::models::users::User;
use crate::schema::tasks;
use crate::task::Task;

#[get("/tasks")]
pub fn tasks_index(user: User, conn: DbConn) -> Result<Json<Vec<Task>>, Status> {
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

#[derive(Deserialize, AsChangeset)]
#[table_name = "tasks"]
pub struct TaskChangeset {
    completed: Option<bool>,
    description: Option<String>,
}

#[patch("/tasks/<id>", format = "application/json", data = "<task>")]
pub fn tasks_update(
    id: i32,
    task: Json<TaskChangeset>,
    conn: DbConn,
) -> Result<Json<Task>, Status> {
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
pub fn tasks_delete(id: i32, conn: DbConn) -> Result<Status, Status> {
    match tasks::table.find(id).get_result::<Task>(&*conn) {
        Ok(_) => diesel::delete(tasks::table.find(id))
            .execute(&*conn)
            .map(|_| Status::NoContent)
            .map_err(|_| Status::InternalServerError),
        Err(error) => match error {
            Error::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}
