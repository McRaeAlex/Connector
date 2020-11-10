use connector::route;
use connector::{connection::Connection, Server};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn handle_root(conn: Connection, id: String) {
    conn.send_resp(hyper::http::StatusCode::OK, format!("{}", id))
        .expect("Failed");
}

fn route(conn: Connection) {
    route!(
        conn,
        hyper::Method::GET,
        "/user/:id",
        |conn: Connection, id: String| {
            conn.send_resp(hyper::http::StatusCode::OK, format!("User id:{}", id))
                .expect("Failed to send");
        }
    );

    route!(conn, hyper::Method::GET, "/:id", handle_root);

    conn.send_resp(hyper::http::StatusCode::NOT_FOUND, "Page not found")
        .expect("Failed to send the message");
}

#[tokio::main]
async fn main() {
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // // let app = App {db_conn: "".into()};
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8888);
    let srv = Server::new(addr, route);

    let fut = srv.start();
    fut.await.expect("Something went wrong");

    println!("Hello");
}
