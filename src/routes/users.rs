use crate::db;
use crate::types::User;
use serde_json::json;
use std::{io::Write, net::TcpStream};

pub fn all_users(mut _stream: TcpStream) {
    let mut client = db::connect();
    let result = client.query("SELECT * FROM users;", &[]);
    println!("{:#?}", result);
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
    let result_json = json!({
        "isSuccess":true,
        "users":users
    });
    let mut response = String::from("HTTP/1.1 200 OK\r\n\r\n");
    response.push_str(result_json.to_string().as_str());
    if let Err(e) = _stream.write((response).as_bytes()) {
        println!("Error writing to TcpStream: {}", e);
    }
}
