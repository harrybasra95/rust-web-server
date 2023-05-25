use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn create_db_pool() -> Result<Pool<Postgres>, Error> {
    let server_url = std::env::var("PG_URL").expect("PG_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&server_url)
        .await?;
    Ok(pool)
}
