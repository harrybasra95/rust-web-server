use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use dotenv::dotenv;

mod routes;
mod db;
mod types;

#[derive(Debug)]
struct Route {
    method: String,
    url: String,
    handler: fn(TcpStream),
}

fn main() {
    dotenv().ok();

    let listner = TcpListener::bind("localhost:3000").unwrap();

    let mut routes: Vec<Route> = Vec::new();
    create_routes(&mut routes);

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        router(stream, &mut routes);
    }
}

fn router(mut stream: TcpStream, routes: &mut Vec<Route>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line: Vec<String> = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let request_method = request_line[0].to_string();
    let request_url = request_line[1].to_string();

    let route = routes
        .iter()
        .find(|route| route.url == request_url && route.method == request_method);

    (route.unwrap().handler)(stream);

   
}

fn create_routes(routes: &mut Vec<Route>) {
    routes.push(Route {
        method: String::from("GET"),
        url: String::from("/users"),
        handler: routes::users::all_users,
    });
}
