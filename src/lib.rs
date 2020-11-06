use std::net::{IpAddr, SocketAddr};

use hyper::http::{ HeaderMap, Method, StatusCode};
use hyper::server::conn;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::Body;
use hyper::Error;
use hyper::Request;
use hyper::Response;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::oneshot;

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
    C: Fn(Connection) + Send + Copy + Sync + 'static,
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
        let connector_service_fn = make_service_fn(move |sock: &conn::AddrStream| {
            let connector = connector.clone();
            let socket = sock.remote_addr();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let connector = connector.clone();
                    handle_request(req, socket, connector.connection_handler) // TODO: we need some way to tell rust that this value lives forever
                }))
            }
        });

        println!("Started the server!");
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

#[derive(Debug)]
pub struct Connection {
    // REQUEST FIELDS
    host: String,
    method: Method,
    path: String, // maybe use the path type
    port: u16,
    remote_ip: IpAddr,
    req_headers: HeaderMap,
    scheme: String,         
    query_string: String,
    req_body: Body,

    // RESPONSE FIELDS
    status: Option<StatusCode>,
    resp_body: Option<Body>, // Probably going to need my own type which can be converted into a Body and is clonable
    resp_cookies: Option<()>,
    resp_headers: HeaderMap,

    // OTHER FIELDS
    send: Option<oneshot::Sender<Response<Body>>>, // This will probably have to change
}

impl Connection {
    pub fn put_status<S>(mut self, status_code: S) -> Self 
    where
        S: Into<StatusCode>
        {
        self.status = Some(status_code.into());
        self
    }

    fn put_body<B>(mut self, body: B) -> Self 
    where
        B: Into<Body>
    {
        self.resp_body = Some(body.into());
        self
    }

    pub fn send_resp<S, B>(self, status_code: S, body: B) -> Result<Self, Box<dyn std::error::Error>> 
    where S: Into<StatusCode>, B: Into<Body>
    {
        self.put_status(status_code).put_body(body).send()
    }

    pub fn send(mut self) -> Result<Self, Box<dyn std::error::Error>> {

        let mut resp = hyper::Response::builder()
            .status(self.status.ok_or("Status not set")?);

        let headers: &mut _ = resp.headers_mut().ok_or("Error setting headers")?;
        *headers = self.resp_headers;

        let body = self.resp_body.unwrap_or(Body::empty()); // I might have to write my own HttpBody implementation that is cloneable but I would prefer not too.
        
        let resp = resp
            .body(body)
            .expect("Failed to create response from connection");

        self.send.ok_or("Response has already been sent")?.send(resp).expect("Failed to send response"); // if unwrap fails then we have failed
        self.send = None;
        Ok(self) // TODO: Error occurs here because we have no way to clone the body
    }
}

impl From<(Request<Body>, SocketAddr)> for Connection {
    // probably change this to (Request<Body>, hyper::Connection) to get the port and ip address
    fn from((req, sock): (Request<Body>, SocketAddr)) -> Self {
        let (parts, body) = req.into_parts();
        Self {
            method: parts.method,
            req_headers: parts.headers,
            scheme: parts.uri.scheme_str().unwrap_or("").into(),
            host: parts.uri.host().unwrap_or("").into(), // maybe we have a default? idk what it would be
            path: parts.uri.path().into(),
            query_string: parts.uri.query().unwrap_or("").into(),
            remote_ip: sock.ip(),
            port: sock.port(),
            req_body: body, 
            send: None,
            status: None,
            resp_body: None,
            resp_headers: HeaderMap::new(),
            resp_cookies: None,
        }
    }
}

