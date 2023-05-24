use std::net::TcpStream;

use crate::{
    db::DbPool,
    routes,
    types::{RequestData, Route},
    utils::{request::get_req_data, response::handle_error},
};

pub mod users;

fn create_routes() -> Vec<Route> {
    let mut routes_vec: Vec<Route> = Vec::new();
    routes_vec.push(Route {
        method: String::from("GET"),
        url: String::from("/users"),
        handler: routes::users::all_users,
    });
    routes_vec.push(Route {
        method: String::from("POST"),
        url: String::from("/users"),
        handler: routes::users::create_user,
    });
    routes_vec
}

pub fn router(mut stream: TcpStream, db_pool: DbPool) {
    let routes = create_routes();

    let req_data = get_req_data(&mut stream);

    if req_data.is_none() {
        return handle_error(&stream, 400, None);
    }
    let RequestData { method, url, .. } = req_data.unwrap();

    let route = routes
        .iter()
        .find(|route| route.url == url && route.method == method);

    if route.is_none() {
        return handle_error(&stream, 404, Some("Route not found"));
    }

    match route {
        Some(route) => (route.handler)(stream, db_pool),
        _ => {
            return handle_error(&stream, 500, None);
        }
    }
}
