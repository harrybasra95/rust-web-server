use std::net::TcpStream;

use crate::{
    db::DbPool,
    routes,
    types::{Request, RequestData, Route},
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

    let req_data = match req_data {
        Some(req_data) => req_data,
        None => {
            return handle_error(&stream, 400, None);
        }
    };

    let route = routes
        .iter()
        .find(|route| route.url == req_data.url && route.method == req_data.method);

    if route.is_none() {
        return handle_error(&stream, 404, Some("Route not found"));
    }

    let request = Request {
        req_data,
        stream,
        db_pool,
    };

    match route {
        Some(route) => (route.handler)(request),
        _ => {
            return handle_error(&request.stream, 500, None);
        }
    }
}
