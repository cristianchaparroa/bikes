/*
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager};
use super::{Connection,Pool, SQLError};
use lazy_static::lazy_static;


// It generates the URL connection using the environmental
// variables.

pub fn get_url() -> String {
    let user = "";
    let password = "";
    let host = "";
    let port = "";
    let database = "";

    format!("postgres://{}:{}@{}:{}/{}", user, password, host, port,database)
}

// It's the pool connection reference.
lazy_static! {
    static ref POOL: Pool = {
        let url = get_url();
        let manager = ConnectionManager::<PgConnection>::new(url);
        Pool::new(manager).expect("Faild to create a DB Pool")
    };
}

// It retrieves the pool connection
pub fn get_connection() -> Result<Connection, SQLError> {
    let pool = POOL.get();
    pool.map_err(|e| SQLError::new(format!("Failed getting db connection: {}", e)))
}

// It initilizes the pool connection to database.
pub fn new_connection() -> Connection {
    lazy_static::initialize(&POOL);
    let result = get_connection();
    let connection = result.unwrap();
    connection
}*/

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;

use super::SQLError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

//embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn get_connection() -> Result<DbConnection, SQLError> {
    POOL.get()
        .map_err(|e| SQLError::new(format!("Failed getting db connection: {}", e)))
}

pub fn init() {
    lazy_static::initialize(&POOL);
}


pub struct Postgres {
    pub conn : DbConnection
}

impl Postgres {
    pub fn new() -> Self {
        let conn =  get_connection().unwrap();
        Postgres{
            conn
        }
    }   
}
