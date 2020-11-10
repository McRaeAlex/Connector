use connector::route;
use connector::{connection::Connection, Server};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

struct App;

impl App {
    fn route(&self, conn: Connection) {
        route!(conn, hyper::Method::GET, "/", |conn: Connection| {
            conn.send_resp(hyper::http::StatusCode::OK, "Wow")
                .expect("Failed");
        });

        conn.send_resp(hyper::http::StatusCode::NOT_FOUND, "Page not found")
            .expect("Failed to send the message");
    }
}

#[tokio::main]
async fn main() {
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // // let app = App {db_conn: "".into()};
    let app = Arc::new(App {});
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8888);
    let srv = Server::new(addr, move |conn: Connection| {
        app.route(conn);
    });

    srv.start().await.expect("Something went wrong");

    println!("Hello");
}
