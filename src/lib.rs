use std::net::SocketAddr;

use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::Body;
use hyper::Error;
use hyper::Request;
use hyper::Response;
use std::sync::Arc;
use std::convert::Infallible;

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
        let builder =  hyper::Server::try_bind(&self.socket_addr).expect("Failed to bind");
        let connector = Arc::new(self);
        let connector_service_fn = make_service_fn(move |_conn: &_| {
            let connector = connector.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let connector = connector.clone();
                    handle_request(req, connector.connection_handler) // TODO: we need some way to tell rust that this value lives forever
                }))
            }
        });
        
        println!("Started the server!");
        builder.serve(connector_service_fn).await
    }

}



async fn handle_request<C>(
    _req: Request<Body>,
    _conn_handler: C,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>>
where
    C: Fn(Connection)
{
    // let mut response: Option<Response<Body>> = None;
    // let mut conn = Connection::from(req);

    // conn.send = Box::new(|conn| {
    //     // Set the response
    //     todo!("Set the response on a send_response");
    // });

    // convert the request into the connection
    // call the connector function
    // convert the response into the response
    Err("".into())
}

pub struct Connection {
    // REQUEST FIELDS
    // _host: String,
    // _method: String,    // HTTP TYPE OF GET POST PUT DELETE...
    // _path: Vec<String>, // maybe use the path type
    // __port: u8,
    // _remote_ip: IpAddr,
    // _req_headers: Vec<String>, // Probably an HTTP Type for this
    // _scheme: String,           // Probably an HTTP Type for this
    // _query_string: String,     // probably an HTTP Type for this

    // RESPONSE FIELDS

    // OTHER FIELDS
    send: Box<dyn Fn(&Self)>,
}

impl Connection {
    fn send_resp(self, status_code: StatusCode) -> Self {
        (self.send)(&self); // Call the function
        self
    }

    //fn set_status_code
    //fn set_resp_body
    //fn send
    // todo: all others
}

enum StatusCode {
    NotFound = 404,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
