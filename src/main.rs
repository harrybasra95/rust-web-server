use dotenv::dotenv;
use routes::router;
use std::net::TcpListener;

mod db;
mod routes;
mod types;
mod utils;

fn main() {
    dotenv().ok();
    let client = db::connect();

    if client.is_none() {
        println!("Error connecting to database");
        return;
    }

    let listner = TcpListener::bind("localhost:3000").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        router(stream);
    }
}
