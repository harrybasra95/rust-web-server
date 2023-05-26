use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as};
use validator::Validate;

use crate::types::{CreateUser, Request, RequestData, User};
use crate::utils::missing_field_err;
use crate::utils::response::{handle_error, send_array, send_data};

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
    let Request {
        req_data,
        stream,
        db_pool,
        ..
    } = req;

    let RequestData { body, .. } = req_data;

    let create_user = CreateUser {
        username: match body.get("username") {
            Some(value) => value.to_string(),
            None => {
                return handle_error(&stream, 403, Some(&missing_field_err("username")));
            }
        },
        password: match body.get("password") {
            Some(value) => value.to_string(),
            None => {
                return handle_error(&stream, 403, Some(&missing_field_err("password")));
            }
        },
        email: match body.get("email") {
            Some(value) => value.to_string(),
            None => {
                return handle_error(&stream, 403, Some(&missing_field_err("email")));
            }
        },
    };

    let is_validated = create_user.validate();
    match is_validated {
        Ok(_) => {}
        Err(e) => {
            return handle_error(&stream, 403, Some(&e.to_string()));
        }
    }

    let result: Result<PgQueryResult, sqlx::Error> =
        query("INSERT INTO users (username, password, email) VALUES ($1, $2, $3)")
            .bind(create_user.username)
            .bind(create_user.password)
            .bind(create_user.email)
            .execute(&db_pool)
            .await;
    match result {
        Ok(_) => send_data(&stream, "User added successfully"),
        Err(e) => println!("{e}"),
    }
}
