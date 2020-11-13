use connector::Server;
use connector::connection::Connection;
use connector::http::{Method, StatusCode};
use connector::{route_async};

use lazy_static::lazy_static;
use handlebars::Handlebars;
use sqlx::PgPool;
use async_once::AsyncOnce;
use serde::Serialize;


lazy_static!{
    static ref HBS: Handlebars<'static> = {
        let mut hbs = Handlebars::new();
        hbs.register_templates_directory(".hbs", "templates")
            .expect("Failed to register template directory");
        hbs
    };

    static ref DB: AsyncOnce<PgPool> = AsyncOnce::new(async {
        PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await.expect("Failed to connect to the database")
    });
}


#[derive(Debug, Serialize)]
struct Index {
    issues: Vec<Issue>,
}
#[derive(sqlx::FromRow, Serialize, Debug)]
struct Issue {
    id: i32,
    title: String,
    body: String,
}

fn send_template<T>(conn: Connection, name: &str, data: &T)
    where
        T: serde::Serialize,
    {
        let home = HBS
            .render(name, data)
            .expect(format!("Failed to render template {}", name).as_str());

        conn.send_resp(StatusCode::OK, home)
            .expect("Failed to send");
    }

async fn get_issue(conn: Connection, id: u32) {
    let issue: Issue = sqlx::query_as("SELECT id, title, body FROM issues WHERE id = $1")
        .bind(id)
        .fetch_one(DB.get().await)
        .await
        .expect("Failed to get issue");
    send_template(conn, "issue", &issue);
}

async fn index(conn: Connection) {
    // Get the issues from the database
    let issues: Vec<Issue> = sqlx::query_as("SELECT id, title, body FROM issues")
        .fetch_all(DB.get().await)
        .await
        .expect("failed to query database");

    // Render them into the template
    send_template(conn, "index", &Index { issues });
}

async fn route(conn: Connection) {
    route_async!(
        conn,
        Method::GET,
        "/issues/:id",
        |conn: Connection, id: u32| { get_issue(conn, id) }
    );
    route_async!(conn, Method::GET, "/", |conn: Connection| {
        return index(conn);
    });
}

#[tokio::main]
async fn main() {
    let srv = Server::new(
        "127.0.0.1:8080".parse().unwrap(),
        move |conn: Connection| {
            tokio::spawn(async move {
                route(conn).await;
            });
        },
    );

    srv.start().await.expect("Failed");
}