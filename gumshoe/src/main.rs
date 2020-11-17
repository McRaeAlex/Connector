use connector::connection::Connection;
use connector::http::{Method, StatusCode};
use connector::{route, route_async};
use connector::Server;

use handlebars::Handlebars;

use sqlx::postgres::PgPool;

use serde::Serialize;

use std::sync::Arc;
use std::fs::{File};
use std::io::ErrorKind;
use std::io::prelude::*;
use std::path::Path;

mod handlers;
mod issue;


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

    fn static_server(&self, conn: Connection, file: String) { // TODO: Escape the .. so we cannot read other assets on the computer
        let path = Path::new("./static").join(file);
        match File::open(path).map_err(|e| e.kind()) {
            Ok(mut file) => {
                let mut s = String::new();
                file.read_to_string(&mut s).expect("Failed to read string");
                conn.send_resp(StatusCode::OK, s).expect("Failed to send");
            },
            Err(ErrorKind::NotFound) => {
                self.handle_404(conn);
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
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
        route_async!(conn, Method::DELETE, "/issues/:id", |conn: Connection, id: u32| self.issues_delete(conn, id));
        route_async!(conn, Method::PUT, "/issues/:id",  |conn: Connection, id: u32| self.issues_edit(conn, id));
        route_async!(conn, Method::GET, "/issues", |conn: Connection| self.issues_index(conn));

        // static file server.
        route!( // TODO: we cannot have /issues and /issues/new.html....
            conn,
            Method::GET,
            "/:file",
            |conn: Connection, file: String| {
                self.static_server(conn, file)
            }
        );
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
