use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub struct Postgres {
    pub pool: Pool,
}

impl Postgres {
    pub fn new() -> Self {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::new(manager).expect("Failed to create db pool");
        Postgres { pool }
    }
}
