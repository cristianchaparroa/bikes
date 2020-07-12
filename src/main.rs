mod application;
use application::SQLConnect;

struct App {
    pool: application::Pool
}

fn main() {
    let postgres =  application::Postgres::new();
    let app = App {pool: postgres.get_connection()};
}   
