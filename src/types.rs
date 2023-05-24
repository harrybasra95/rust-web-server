use std::net::TcpStream;

use serde::{Deserialize, Serialize};

use crate::db::DbPool;

#[derive(Debug, Serialize, Clone, Default, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub method: String,
    pub url: String,
    pub handler: fn(TcpStream, DbPool),
}

#[derive(Debug, Serialize, Clone, Default, Deserialize)]
pub struct RequestData {
    pub method: String,
    pub url: String,
    pub query_params: Vec<(String, String)>,
}
