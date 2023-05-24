use db::create_db_pool;
use dotenv::dotenv;
use routes::router;
use std::net::TcpListener;

mod db;
mod routes;
mod types;
mod utils;

fn main() {
    dotenv().ok();
    let db_pool = match create_db_pool() {
        Ok(pool) => pool,
        Err(_) => {
            println!("Error connecting to database");
            return;
        }
    };

    let listner = TcpListener::bind("localhost:3000").unwrap();

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => router(stream, db_pool.clone()),
            Err(_) => {
                println!("Error accepting connection");
                continue;
            }
        }
    }
}
