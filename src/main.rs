use db::create_db_pool;
use dotenv::dotenv;
use routes::router;
use std::net::TcpListener;

mod db;
mod routes;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_pool = create_db_pool().await.unwrap();

    let listner = TcpListener::bind("localhost:3000").unwrap();

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => router(stream, db_pool.clone()).await,
            Err(_) => {
                println!("Error accepting connection");
                continue;
            }
        }
    }
    drop(db_pool);
}
