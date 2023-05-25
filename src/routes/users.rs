use sqlx::query_as;

use crate::types::{Request, RequestData, User};
use crate::utils::response::send_array;

pub async fn all_users(req: Request) {
    let Request {
        db_pool, stream, ..
    } = req;

    let result: Result<Vec<User>, sqlx::Error> =
        query_as("SELECT * FROM users;").fetch_all(&db_pool).await;

    match result {
        Ok(users) => send_array(&stream, &users),
        Err(e) => println!("{e}"),
    }
}

pub async fn create_user(req: Request) {
    let Request { req_data, .. } = req;

    let RequestData { body, .. } = req_data;
    println!("{body:?}");
}
