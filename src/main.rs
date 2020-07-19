#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;

mod schema;

#[database("info")]
pub struct DbConn(diesel::MysqlConnection);

fn main() {
}
