use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use super::{SQLConnect, Pool};

pub struct  Postgres {

    pool :  Box<Pool>
}

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

impl<'a> Postgres {

    pub fn new() -> Self {

        let url = get_url();
    
        let manager = ConnectionManager::<PgConnection>::new(url);
        let result = Pool::new(manager);
        let pool = result.unwrap();
    
        let b = Box::new(pool);
        Postgres{pool:b}
    }
}


impl SQLConnect for Postgres {

    fn get_connection(&self) -> Pool {
        *self.pool
    }
}
