use connector::connection::Connection;
use connector::http::{Method, StatusCode};
use connector::route;
use connector::Server;

use handlebars::Handlebars;
use sqlx::postgres::PgPool;

use std::sync::Arc;

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

    fn route(&self, conn: Connection) {
        route!(
            conn,
            Method::GET,
            "/issues/:id",
            |conn: Connection, id: usize| {
                self.get_issue(conn, id);
            }
        );
        route!(conn, Method::GET, "/", |conn: Connection| self
            .send_template(conn, "index", &String::from("Hello")));
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
            app.route(conn);
        },
    );

    srv.start().await.expect("Failed");
}
