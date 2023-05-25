use sqlx::query_as;

use crate::types::{Request, User};
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

pub fn create_user(req: Request) {}
