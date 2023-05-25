use crate::db::DbPool;
use crate::types::{Request, User};
use crate::utils::response::send_array;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn all_users(req: Request) {
    let Request {
        db_pool, stream, ..
    } = req;
    let mut client = db_pool.get().unwrap();

    let result = client.query("SELECT * FROM users;", &[]);
    if let Err(e) = result {
        return println!("{e}");
    }

    let result = result.unwrap();

    let mut users: Vec<User> = Vec::new();
    for row in result {
        let id: i32 = row.get(0);
        let username: String = row.get(1);
        let password: String = row.get(2);
        let email: String = row.get(3);
        let user = User {
            id,
            username,
            password,
            email,
        };
        users.push(user);
    }
    send_array::<User>(&stream, &users);
}

pub fn create_user(req: Request) {}
