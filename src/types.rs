use std::{collections::HashMap, future::Future, net::TcpStream, pin::Pin};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{constants::enums::RequestTypes, db::DbPool};

#[derive(Debug, Serialize, Clone, Default, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
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
}

#[derive(Debug)]
pub struct Request {
    pub req_data: RequestData,
    pub stream: TcpStream,
    pub db_pool: DbPool,
}
