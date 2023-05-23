use dotenv::dotenv;
use std::net::{TcpListener, TcpStream};
use types::{RequestData, Route};
use utils::{request::get_req_data, response::handle_error};

mod db;
mod routes;
mod types;
mod utils;

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
    let req_data = get_req_data(&mut stream);

    if req_data.is_none() {
        return handle_error(&stream, 400, None);
    }
    let RequestData { method, url } = req_data.unwrap();

    let route = routes
        .iter()
        .find(|route| route.url == url && route.method == method);

    if route.is_none() {
        return handle_error(&stream, 404, Some("Route not found"));
    }

    match route {
        Some(route) => (route.handler)(stream),
        _ => {
            return handle_error(&stream, 500, None);
        }
    }
}

fn create_routes(routes: &mut Vec<Route>) {
    routes.push(Route {
        method: String::from("GET"),
        url: String::from("/users"),
        handler: routes::users::all_users,
    });
    routes.push(Route {
        method: String::from("POST"),
        url: String::from("/users"),
        handler: routes::users::create_user,
    })
}
