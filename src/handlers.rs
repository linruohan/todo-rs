use std::sync::Arc;

use crate::models::{NewTodo, Todo, UpdateTodo};
use crate::schema::todos;
use crate::schema::todos::id;
use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode};
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

pub type DbPool = Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>;

pub async fn create_todo(
    State(db): State<DbPool>,
    Json(new_todo): Json<NewTodo>,
) -> Result<String, (StatusCode, &'static str)> {
    let mut conn = db
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .execute(&mut conn)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "failed to create todo"))?;

    Ok("create success".to_owned())
}

pub async fn get_todos(State(db): State<DbPool>) -> (StatusCode, Json<Vec<Todo>>) {
    let mut conn = db
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    let results = todos::table
        .load::<Todo>(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .expect("must be todos");

    (StatusCode::OK, Json(results))
}

pub async fn get_todo(
    Path(todo_id): Path<i32>,
    State(db): State<DbPool>,
) -> Result<String, (StatusCode, &'static str)> {
    let mut conn = db
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    todos::table
        .filter(id.eq(todo_id))
        .first::<Todo>(&mut conn)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "get todo failed"))?;
    Ok("get todo success".to_owned())
}

pub async fn update_todo(
    Path(todo_id): Path<i32>,
    State(db): State<DbPool>,
    Json(update_todo): Json<UpdateTodo>,
) -> Result<String, (StatusCode, &'static str)> {
    let mut conn = db
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    diesel::update(todos::table.filter(id.eq(todo_id)))
        .set(&update_todo)
        .returning(Todo::as_returning())
        .get_result(&mut conn)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "failed update todo"))?;
    Ok("update success".to_owned())
}

pub async fn delete_todo(Path(todo_id): Path<i32>, State(db): State<DbPool>) -> StatusCode {
    let mut conn = db
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    let _ = diesel::delete(todos::table.filter(id.eq(todo_id)))
        .execute(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    StatusCode::NO_CONTENT
}
