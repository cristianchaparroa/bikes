
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;


pub trait SQLConnect {
    
    fn get_connection(&self) -> Pool;
}