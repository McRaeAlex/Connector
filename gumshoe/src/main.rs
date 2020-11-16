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
        T: Serialize,
    {
        let home = self
            .hbs
            .render(name, data)
            .expect(format!("Failed to render template {}", name).as_str());

        conn.send_resp(StatusCode::OK, home)
            .expect("Failed to send");
    }

    async fn route(&self, conn: Connection) {
        // resource_async!(conn, "/issues", obj || Nothing) if a third arg isn't present just generate the functions not hanging off an object
        route_async!(conn, Method::POST, "/issues/new", |conn: Connection| {
            self.issues_new(conn)
        });
        route_async!(
            conn,
            Method::GET,
            "/issues/:id",
            |conn: Connection, id: u32| self.issues_show(conn, id)
        );
        route_async!(conn, Method::GET, "/issues", |conn: Connection| self.issues_index(conn));

        // static file server.
        route_async!(conn, Method::GET, "/:file", |conn: Connection, _file: String| {
            return self.issues_index(conn);
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
