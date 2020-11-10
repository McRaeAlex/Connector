use super::Connection;

use hyper::http::Method;

#[derive(Debug)]
enum PathOrVar {
    Path(String),
    Var,
}

fn parse_path(p: String) -> Vec<PathOrVar> {
    let mut chunks = vec![];
    let mut accumulator = String::new();
    let mut var = false; // if var is true we ignore the input

    for ch in p.chars() {
        if !var && ch == ':' {
            var = true;
            chunks.push(PathOrVar::Path(accumulator));
            chunks.push(PathOrVar::Var);
            accumulator = String::new();
            continue;
        }

        if var && ch == '/' {
            var = false;
        }

        if !var {
            accumulator.push(ch);
        }
    }

    chunks
}

impl Connection {
    pub fn match_method(&self, method: Method) -> bool {
        self.method == method
    }

    // TODO: we currently only support matching the first part of a path.
    // This means if we write route!(conn, Method::GET, "/user/:id", some_handler)
    // it will match "/user/:id/anything"
    pub fn match_path(&self, path: &str) -> Option<impl Iterator<Item = String>> {
        let mut vars: Vec<String> = vec![];

        // break the path into chunks that can be matched on
        // ie: /user/:id/comments/:comment_id -> [/user/, :id, /comments/, :comment_id]
        // then we consume through the self.path matching the largest chunks we can
        let chunks = parse_path(path.to_string());
        let mut conn_path = self.path.clone();
        for chunk in chunks.iter() {
            match chunk {
                PathOrVar::Path(s) => {
                    let l = s.len();
                    if l > conn_path.len() {
                        return None;
                    } 

                    let rest =  conn_path.split_off(l);
                    
                    if *s != conn_path {
                        return None;
                    }
                    conn_path = rest; // set the conn_path to the non consumed portion
                }
                PathOrVar::Var => {
                    // find the closest '/'
                    // split the string off at that location
                    // put that value into the array
                    let index = conn_path.find('/').unwrap_or(conn_path.len());

                    let rest = conn_path.split_off(index);
                    vars.push(conn_path);
                    conn_path = rest;
                },
            };
        }

        Some(vars.into_iter())
    }
}
