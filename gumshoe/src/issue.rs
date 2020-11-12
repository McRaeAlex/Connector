use serde::Serialize;
#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Issue {
    id: i32,
    title: String,
    body: String,
}
