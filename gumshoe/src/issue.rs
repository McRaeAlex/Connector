use connector::connection::Connection;
use connector::http::{
    header::{HeaderName, HeaderValue},
    StatusCode,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Index {
    issues: Vec<Issue>,
}
#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Issue {
    id: i32,
    title: String,
    body: String,
}

use crate::App;

impl<'a> App<'a> {
    pub async fn issues_index(&self, conn: Connection) {
        // Get the issues from the database
        let issues: Vec<Issue> = sqlx::query_as("SELECT id, title, body FROM issues")
            .fetch_all(&self.db)
            .await
            .expect("failed to query database"); // Internal server error

        // Render them into the template
        self.send_template(conn, "index", &Index { issues });
    }

    pub async fn issues_show(&self, conn: Connection, id: u32) {
        let issue: Issue = match sqlx::query_as("SELECT id, title, body FROM issues WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await
        {
            Err(e) => {
                self.handle_error(conn, e);
                return;
            }
            Ok(val) => val,
        };

        self.send_template(conn, "issues/show", &issue);
    }

    pub async fn issues_new(&self, conn: Connection) {
        // parse the body as a form
        println!("{:?}", conn.read_body());
        // parse that form into vars
        // insert into database
    }

    pub async fn issues_delete(&self, conn: Connection, id: u32) {
        match sqlx::query("DELETE FROM issues WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                self.handle_error(conn, e);
                return;
            }
        }

        conn.put_status(StatusCode::PERMANENT_REDIRECT)
            .put_resp_header(
                HeaderName::from_static("Location"),
                HeaderValue::from_static("/issues"),
            )
            .send().expect("Failed to send");
    }

    pub async fn issues_edit(&self, conn: Connection, id: u32) {
        todo!();
    }
}
