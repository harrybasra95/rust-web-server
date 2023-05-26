use std::{collections::HashMap, future::Future, net::TcpStream, pin::Pin};

use crate::{constants::enums::RequestTypes, db::DbPool};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Clone, Default, Deserialize, FromRow, Validate)]
pub struct User {
    pub id: i32,
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 3))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}

type RouteHandler = Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = ()>>>>;

pub struct Route {
    pub method: RequestTypes,
    pub url: String,
    pub handler: RouteHandler,
}

#[derive(Debug, Serialize, Clone, Default, Deserialize)]
pub struct RequestData {
    pub method: RequestTypes,
    pub url: String,
    pub query_params: HashMap<String, String>,
    pub body: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub url_params: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Request {
    pub req_data: RequestData,
    pub stream: TcpStream,
    pub db_pool: DbPool,
}

#[derive(Debug, Serialize, Clone, Default, Deserialize, FromRow, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(length(min = 3))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}
