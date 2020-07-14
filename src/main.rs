#[macro_use]
extern crate diesel;

mod application;
mod bikes;

use application::provider::{DbConnection ,Postgres};

struct app {
    conn :DbConnection
}

fn main() {

    let postgres = Postgres::new();
    let connection = postgres.conn;
    let application = app{
        conn: connection
    };
}
