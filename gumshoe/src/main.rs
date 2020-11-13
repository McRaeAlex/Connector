use connector::connection::Connection;
use connector::http::{Method, StatusCode};
use connector::route_async;
use connector::Server;

use handlebars::Handlebars;

use sqlx::postgres::PgPool;

use serde::Serialize;

use std::sync::Arc;

mod handlers;
mod issue;

use issue::Issue;

#[derive(Debug, Serialize)]
struct Index {
    issues: Vec<Issue>,
}

fn _static_server(_conn: Connection, _file: String) {
    // read file into the string
    // set the content type based on the file extension
}

struct App<'a> {
    // probably some database connection pool
    db: sqlx::PgPool,
    hbs: Handlebars<'a>,
}

impl<'a> App<'a> {
    async fn new() -> App<'a> {
        let mut hbs = Handlebars::new();
        hbs.register_templates_directory(".hbs", "templates")
            .expect("Failed to register template directory");

        let db = PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
            .await
            .expect("failed to creat the postgres pool");
        App { db, hbs }
    }

    fn send_template<T>(&self, conn: Connection, name: &str, data: &T)
    where
        T: serde::Serialize,
    {
        let home = self
            .hbs
            .render(name, data)
            .expect(format!("Failed to render template {}", name).as_str());

        conn.send_resp(StatusCode::OK, home)
            .expect("Failed to send");
    }

    async fn index(&self, conn: Connection) {
        // Get the issues from the database
        let issues: Vec<Issue> = sqlx::query_as("SELECT id, title, body FROM issues")
            .fetch_all(&self.db)
            .await
            .expect("failed to query database"); // Internal server error

        // Render them into the template
        self.send_template(conn, "index", &Index { issues });
    }

    async fn get_issue(&self, conn: Connection, id: u32) {
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

        self.send_template(conn, "issue", &issue);
    }

    async fn route(&self, conn: Connection) {
        route_async!(
            conn,
            Method::GET,
            "/issues/:id",
            |conn: Connection, id: u32| { self.get_issue(conn, id) }
        );
        route_async!(conn, Method::GET, "/", |conn: Connection| {
            return self.index(conn);
        });
    }
}

#[tokio::main]
async fn main() {
    let app = Arc::new(App::new().await);
    let srv = Server::new(
        "127.0.0.1:8080".parse().unwrap(),
        move |conn: Connection| {
            let app = app.clone();
            tokio::spawn(async move {
                app.route(conn).await;
            });
        },
    );

    srv.start().await.expect("Failed");
}
