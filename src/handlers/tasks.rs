use rocket::State;
use rocket_contrib::json::Json;

use crate::connection;
use crate::schema::tasks;
use crate::task::Task;

#[get("/tasks/<id>")]
pub fn tasks_get(
    id: i32,
    pool: State<connection::PgPool>,
) -> Result<Json<Task>, diesel::result::Error> {
    use diesel::prelude::*;

    let conn = pool.get().unwrap();
    let query_result: QueryResult<Task> = tasks::table.find(id).get_result::<Task>(&conn);
    query_result.map(|task| Json(task))
}
