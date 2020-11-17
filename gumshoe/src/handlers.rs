use connector::connection::Connection;

use serde::Serialize;

use super::App;

#[derive(Debug, Serialize)]
struct FourOhFour {}

impl<'a> App<'a> {
    pub fn handle_error(&self, conn: Connection, error: sqlx::Error) {
        match error {
            sqlx::Error::RowNotFound => self.handle_404(conn),
            _ => todo!("Internal Server error I guess"),
        }
    }

    pub fn handle_404(&self, conn: Connection) {
        self.send_template(conn, "404", &FourOhFour {})
    }
}
