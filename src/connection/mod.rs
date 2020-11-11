mod routing;

use std::net::{IpAddr, SocketAddr};

use hyper::header::{HeaderName, GetAll};
use hyper::http::{HeaderMap, HeaderValue, Method, StatusCode, Version};

use hyper::Body;
use hyper::Request;
use hyper::Response;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Connection {
    // REQUEST FIELDS
    version: Version,
    host: String,
    method: Method,
    path: String,
    port: u16,
    remote_ip: IpAddr,
    req_headers: HeaderMap,
    scheme: String,
    // TODO: Consider making this a QueryMap struct or something it should just be empty if the query string is empty. This may be inflexible for some usecases I can't think of right now
    query_string: String,
    req_body: Body,

    // RESPONSE FIELDS
    status: Option<StatusCode>,
    resp_body: Option<Vec<u8>>,
    resp_cookies: Option<()>,
    resp_headers: HeaderMap,

    // OTHER FIELDS
    pub(crate) send: Option<oneshot::Sender<Response<Body>>>,
}

// setters
impl Connection {
    /// Sets the status code setting or overwriting the old status code
    pub fn put_status<S>(mut self, status_code: S) -> Self
    where
        S: Into<StatusCode>,
    {
        self.status = Some(status_code.into());
        self
    }

    /// Puts a value into the headermap. Will overwrite the existing value.
    pub fn put_req_header<K, V>(mut self, key: K, val: V) -> Self
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self.req_headers.insert(key.into(), val.into());
        self
    }

    /// Puts a value into the headermap. Will Overwrite the existing value.
    pub fn put_resp_header<K, V>(mut self, key: K, val: V) -> Self
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self.resp_headers.insert(key.into(), val.into());
        self
    }

    /// A convenience function for setting the content type header
    pub fn put_resp_content_type<V>(mut self, c_type: V) -> Self
    where
        V: Into<HeaderValue>,
    {
        self.resp_headers
            .insert(hyper::header::CONTENT_TYPE, c_type.into());
        self
    }

    /// TODO
    pub fn put_resp_cookie(self) -> Self {
        todo!(); // TODO
    }

    /// Sets or overwrites the body of the response
    pub fn put_body<B>(mut self, body: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        self.resp_body = Some(body.into());
        self
    }
}

// delete
impl Connection {
    /// Deletes a header key value pair from the request HeaderMap
    pub fn delete_req_header<K>(mut self, key: K) -> Self
    where
        K: Into<HeaderName>,
    {
        self.req_headers.remove(key.into());
        self
    }

    /// Deletes a header key value pair from the response HeaderMap
    pub fn delete_resp_header<K>(mut self, key: K) -> Self
    where
        K: Into<HeaderName>,
    {
        self.resp_headers.remove(key.into());
        self
    }

    pub fn delete_resp_cookie(self) -> Self {
        todo!(); // TODO
    }
}

// fetch
impl Connection {
    pub fn fetch_query_params(self) {
        todo!(); // TODO
    }

    pub fn fetch_cookies(self) {
        todo!(); // TODO
    }
}

// get
impl Connection {
    pub fn get_http_protocol(&self) -> Version
    {
        self.version
    }

    pub fn get_req_header<K>(&self, key: K) -> GetAll<HeaderValue>
    where K: Into<HeaderName>
    {
        self.req_headers.get_all(key.into())
    }

    pub fn get_resp_header<K>(&self, key: K) -> GetAll<HeaderValue>
    where K: Into<HeaderName>
    {
        self.resp_headers.get_all(key.into())
    }
}

// send
impl Connection {
    /// Sends a JSON Response setting the correct content type
    pub fn send_json<S, B>(
        self,
        status_code: S,
        body: &B,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        S: Into<StatusCode>,
        B: serde::Serialize,
    {
        self.put_status(status_code)
            .put_resp_content_type(HeaderValue::from_static("application/json"))
            .put_body(serde_json::to_string(&body).unwrap())
            .send()
    }

    /// Sends a response. Does not set the content type
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

    /// Sends the response. It may fail if the response has already been sent or
    /// The response is invalid
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

    pub fn halt(self) {
        // consumes itself
        // we should probably send some internal server error if the 
        // self.send is still active so we don't leak resources
    }
}

impl From<(Request<Body>, SocketAddr)> for Connection {
    fn from((req, sock): (Request<Body>, SocketAddr)) -> Self {
        let version = req.version();
        let (parts, body) = req.into_parts();
        Self {
            version,
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
