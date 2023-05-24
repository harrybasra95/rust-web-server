use std::net::TcpStream;

use serde::{Deserialize, Serialize};

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
    pub handler: fn(TcpStream),
}

#[derive(Debug, Serialize, Clone, Default, Deserialize)]
pub struct RequestData {
    pub method: String,
    pub url: String,
    pub query_params: Vec<(String, String)>,
}
