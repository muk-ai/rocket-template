use diesel::prelude::*;
use diesel::result::Error;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Deserialize;

use crate::connection::DbConn;
use crate::log::{write_info, TraceContext};
use crate::models::tasks::Task;
use crate::models::users::User;
use crate::schema::tasks;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Mount /tasks", |rocket| async {
        rocket.mount(
            "/tasks",
            routes![
                tasks_index,
                tasks_get,
                tasks_post,
                tasks_update,
                tasks_delete
            ],
        )
    })
}

#[get("/")]
fn tasks_index(
    user: User,
    conn: DbConn,
    trace: Option<&TraceContext>,
) -> Result<Json<Vec<Task>>, Status> {
    write_info("この変更は反映されています！ 13:01", trace);
    write_info("/tasks called", trace);
    let query_result: QueryResult<Vec<Task>> = Task::belonging_to(&user).load::<Task>(&*conn);
    query_result
        .map(Json)
        .map_err(|_error| Status::InternalServerError)
}

#[get("/<id>")]
fn tasks_get(user: User, id: i32, conn: DbConn) -> Result<Json<Task>, Status> {
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
struct TaskDescriptionData {
    description: String,
}

#[derive(Insertable)]
#[table_name = "tasks"]
struct InsertableTask {
    description: String,
    completed: bool,
    user_id: uuid::Uuid,
}

#[post("/", format = "application/json", data = "<task>")]
fn tasks_post(user: User, task: Json<TaskDescriptionData>, conn: DbConn) -> Result<Status, Status> {
    let query_result = diesel::insert_into(tasks::table)
        .values(&InsertableTask {
            description: task.into_inner().description,
            user_id: user.id,
            completed: false,
        })
        .get_result::<Task>(&*conn);
    query_result
        .map(|_task| Status::Created)
        .map_err(|_error| Status::InternalServerError)
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "tasks"]
struct TaskChangeset {
    completed: Option<bool>,
    description: Option<String>,
}

#[patch("/<id>", format = "application/json", data = "<task>")]
fn tasks_update(
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
    query_result.map(Json).map_err(|error| match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    })
}

#[delete("/<id>")]
fn tasks_delete(user: User, id: i32, conn: DbConn) -> Result<Status, Status> {
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
