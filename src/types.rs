use serde::Serialize;

#[derive(Serialize)]

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}
