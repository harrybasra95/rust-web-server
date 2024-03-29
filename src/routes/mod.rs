use std::net::TcpStream;

use crate::{
    constants::enums::RequestTypes,
    db::DbPool,
    routes,
    types::{Request, Route},
    utils::{
        request::{get_req_data, match_and_extract_url_params},
        response::handle_error,
    },
};

pub mod users;

fn create_routes() -> Vec<Route> {
    vec![
        Route {
            method: RequestTypes::GET,
            url: String::from("/users"),
            handler: Box::new(|r: Request| Box::pin(routes::users::all_users(r))),
        },
        Route {
            method: RequestTypes::POST,
            url: String::from("/users"),
            handler: Box::new(|r: Request| Box::pin(routes::users::create_user(r))),
        },
        Route {
            method: RequestTypes::POST,
            url: String::from("/users/:id"),
            handler: Box::new(|r: Request| Box::pin(routes::users::get_one_user(r))),
        },
    ]
}

pub async fn router(mut stream: TcpStream, db_pool: DbPool) {
    let routes = create_routes();

    let req_data = get_req_data(&mut stream);

    let mut req_data = match req_data {
        Some(req_data) => req_data,
        None => {
            return handle_error(&stream, 400, None);
        }
    };

    let route = routes.iter().find(|route| {
        match_and_extract_url_params(&req_data.url, &route.url, &mut req_data.url_params)
            && route.method == req_data.method
    });

    if route.is_none() {
        return handle_error(&stream, 404, Some("Route not found"));
    }

    let request = Request {
        req_data,
        stream,
        db_pool,
    };

    match route {
        Some(route) => {
            (route.handler)(request).await;
        }
        _ => {
            return handle_error(&request.stream, 500, None);
        }
    }
}
