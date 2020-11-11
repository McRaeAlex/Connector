mod routing;

use std::net::{IpAddr, SocketAddr};

use hyper::http::{HeaderMap, Method, StatusCode};

use hyper::Body;
use hyper::Request;
use hyper::Response;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Connection {
    // REQUEST FIELDS
    host: String,
    method: Method,
    path: String, // maybe use the path type
    port: u16,
    remote_ip: IpAddr,
    pub req_headers: HeaderMap,
    scheme: String,
    query_string: String,
    req_body: Body,

    // RESPONSE FIELDS
    status: Option<StatusCode>,
    resp_body: Option<Vec<u8>>,
    resp_cookies: Option<()>,
    pub resp_headers: HeaderMap,

    // OTHER FIELDS
    pub(crate) send: Option<oneshot::Sender<Response<Body>>>,
}


// setters implementations
impl Connection {
    pub fn put_status<S>(mut self, status_code: S) -> Self
    where
        S: Into<StatusCode>,
    {
        self.status = Some(status_code.into());
        self
    }

    pub fn put_header(self) -> Self {
        self
    }

    pub fn put_body<B>(mut self, body: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        self.resp_body = Some(body.into());
        self
    }
}

// send implementations
impl Connection {
    pub fn send_json<S, B>(self, status_code: S, body: &B) -> Result<Self, Box<dyn std::error::Error>>
    where
        S: Into<StatusCode>,
        B: serde::Serialize
    {
        // TODO: Set the correct header
        self.put_status(status_code).put_body(serde_json::to_string(&body).unwrap()).send()
    }

    pub fn send_resp<S, B>(
        self,
        status_code: S,
        body: B,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        S: Into<StatusCode>,
        B: Into<Vec<u8>>,
    {
        self.put_status(status_code).put_body(body).send()
    }

    pub fn send(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let mut resp = hyper::Response::builder().status(self.status.ok_or("Status not set")?);

        let headers: &mut _ = resp.headers_mut().ok_or("Error setting headers")?;
        *headers = self.resp_headers.clone(); // set the headers

        let body = self.resp_body.clone(); // TODO: try to remove this clone

        let resp = resp
            .body(body.map_or(Body::empty(), |v| v.into()))
            .expect("Failed to create response from connection");

        self.send
            .ok_or("Response has already been sent")?
            .send(resp)
            .expect("Failed to send response"); // if unwrap fails then we have failed
        self.send = None;

        Ok(self)
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
