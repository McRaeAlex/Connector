use connector::connection::Connection;
use connector::http::{Method, StatusCode};
use connector::Server;
use connector::{route, route_async};

use handlebars::Handlebars;

use sqlx::postgres::PgPool;
use sqlx::prelude::*;

use serde_json::json;

use std::sync::Arc;

mod issue;

use issue::Issue;

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

        let db = PgPool::new(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
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
            .expect("failed to query database");

        // Render them into the template
        self.send_template(conn, "index", &json!({ "issues": issues }));
    }

    async fn route(&self, conn: Connection) {
        route!(
            conn,
            Method::GET,
            "/issues/:id",
            |conn: Connection, id: usize| {
                self.get_issue(conn, id);
            }
        );
        route_async!(conn, Method::GET, "/", move |conn: Connection| {
            return self.index(conn);
        });
    }

    fn get_issue(&self, _conn: Connection, _id: usize) {
        // retrieve the issue from the database then return it
        todo!();
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
                // Unfortunate but nessecary
                app.route(conn).await;
            });
        },
    );

    srv.start().await.expect("Failed");
}
