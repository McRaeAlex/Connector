use serde::Serialize;
#[derive(sqlx::FromRow, Serialize)]
pub struct Issue {
    id: i32,
    title: String,
    body: String,
}
