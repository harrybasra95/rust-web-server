use serde::Serialize;
use serde_json::json;
use std::io::Write;
use std::net::TcpStream;

pub fn send_array<T>(mut stream: &TcpStream, data: &[T])
where
    T: Serialize,
{
    let result_json = json!({
        "isSuccess":true,
        "items":data
    });
    let response: String = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\n\r\n{}",
        result_json
    );
    if let Err(e) = stream.write_all((response).as_bytes()) {
        println!("Error writing to TcpStream: {}", e);
    }
}

pub fn send_data<T>(mut stream: &TcpStream, data: T)
where
    T: Serialize,
{
    let result_json = json!({
        "isSuccess":true,
        "item":data
    });
    let response: String = format!("HTTP/1.1 200 OK\r\n\r\n{}", result_json);
    if let Err(e) = stream.write((response).as_bytes()) {
        println!("Error writing to TcpStream: {}", e);
    }
}

pub fn handle_error(mut stream: &TcpStream, status_code: u16, e: Option<&str>) {
    let error_message = match e {
        Some(value) => value,
        _ => "Internal Server Error",
    };
    let result_json = json!({
        "isSuccess":false,
        "error":error_message
    });
    let response: String = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json; charset=utf-8\r\n\r\n{}",
        status_code,
        get_status_text(status_code),
        result_json
    );
    if let Err(e) = stream.write((response).as_bytes()) {
        println!("Error writing to TcpStream: {}", e);
    }
}

fn get_status_text(status_code: u16) -> &'static str {
    match status_code {
        200 => "OK",
        400 => "Bad Request",
        403 => "Validation Error",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}
