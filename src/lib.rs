use hyper::server::conn;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Request, Response};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot;

pub mod connection;
pub use hyper::http;
pub use route::{route, route_async};

use connection::Connection;

pub struct Server<C>
where
    C: Fn(Connection),
{
    socket_addr: SocketAddr,
    connection_handler: C,
    // error handler
}

impl<C> Server<C>
where
    C: Fn(Connection) + Clone + Send + Sync + 'static,
{
    pub fn new(socket_addr: SocketAddr, connection_handler: C) -> Self {
        Server {
            socket_addr,
            connection_handler,
        }
    }

    pub async fn start(self) -> Result<(), Error> {
        let builder = hyper::Server::try_bind(&self.socket_addr).expect("Failed to bind");
        let connector = Arc::new(self);
        let local = connector.clone();
        let connector_service_fn = make_service_fn(move |sock: &conn::AddrStream| {
            let connector = connector.clone();
            let socket = sock.remote_addr();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let connector = connector.clone();
                    handle_request(req, socket, connector.connection_handler.clone())
                }))
            }
        });

        println!("Starting the server {}", local.socket_addr);
        builder.serve(connector_service_fn).await
    }
}

async fn handle_request<C>(
    req: Request<Body>,
    socket: SocketAddr,
    conn_handler: C,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>>
where
    C: Fn(Connection) + Send + Sync + 'static,
{
    // Effectively what needs to happen is we need two threads of control
    // one returns the response when response is Some and one that runs the
    // connection handler which should at some point in the future send the
    // response. Unsure how to do this

    let (tx, rx): (
        oneshot::Sender<Response<Body>>,
        oneshot::Receiver<Response<Body>>,
    ) = oneshot::channel();

    // spawn the task
    tokio::spawn(async move {
        let mut conn: Connection = (req, socket).into();
        conn.send = Some(tx);
        conn_handler(conn);
    });

    let resp: Result<Response<Body>, oneshot::error::RecvError> = rx.await;
    let resp = match resp {
        Ok(val) => val,
        Err(e) => {
            println!("Failed to set the response");
            return Err(e.into());
        }
    };

    Ok(resp)
}
