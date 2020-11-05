use connector::{Connection, Server};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use hyper::rt::*;

fn route(_conn: Connection) {
    println!("Called");
}
#[tokio::main]
async fn main() {
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // // let app = App {db_conn: "".into()};
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8888);
    let srv = Server::new(addr, route);

    let fut = srv.start(); // TODO: this call fails
    fut.await.expect("Something went wrong");

    println!("Hello");
}
