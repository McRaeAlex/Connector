use test_path_dsl::route;

struct Conn;

impl Conn {
    fn match_method(&self, _method: usize) -> bool {
        true
    }

    fn match_path<'a>(&self, _path: &'a str) -> Option<Vec<usize>> {
        Some(vec![1, 2])
    }
}

fn handle_route(_conn: Conn) {
    todo!();
}

fn main() {
    let conn = Conn{};
    let get = 1;
    route!(conn, get, "/user/:id/comments/:comment_id", handle_route);
}