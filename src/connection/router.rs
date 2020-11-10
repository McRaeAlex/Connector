use super::Connection;

use hyper::http::Method;

impl Connection {
    pub fn match_method(&self, method: Method) -> bool {
        self.method == method
    }

    pub fn match_path<P>(&self, _path: P) -> Option<impl Iterator<Item = String>>
    where
        P: Into<String>,
    {
        // route is a String for now but we need variable routes with captures probably regex tbh
        // parse the path string

        // parse the self string
        let res: Option<_> = Some(vec!["Hello".into()].into_iter());
        res
    }
}
