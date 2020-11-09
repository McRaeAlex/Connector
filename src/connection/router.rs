use super::Connection;

use hyper::http::Method;

#[macro_export]
macro_rules! route {
    ($conn:ident, $method:expr, $path:expr, $func:expr) => {
        if $conn.match_method($method) {
            if let Some(_vars) = $conn.match_path($path) {
                // Parse the path into unique variables here
                // like mentioned in the notes.

                $func($conn);
                return;
            }
        }
    };
}


impl Connection {
    pub fn match_method(&self, method: Method) -> bool {
        self.method == method
    }

    pub fn match_path<P>(&self, _path: P) -> Option<impl Iterator<Item = String>>
    where
        P: Into<String>,
    {
        // route is a String for now but we need variable routes with captures probably regex tbh
        let res: Option<_> = Some(vec!["Hello".into()].into_iter());
        res
    }
}
