use std::error::Error;

use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

pub fn create_db_pool() -> Result<DbPool, Box<dyn Error>> {
    let server_url = std::env::var("PG_URL").expect("PG_URL must be set");
    let manager = PostgresConnectionManager::new(server_url.parse().unwrap(), NoTls);
    let pool = Pool::builder().max_size(10).build(manager)?;
    Ok(pool)
}
