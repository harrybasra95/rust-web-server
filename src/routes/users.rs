use std::{net::TcpStream, path::Path, fs, io::Write};

pub fn all_users(mut stream : TcpStream) {
     let file_path = Path::new("public/index.html");

    let index_html_file = fs::read_to_string(file_path).unwrap();
    let length = index_html_file.len();
    let status_line = "HTTP/1.1 200 OK";

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{index_html_file}");

    stream.write_all(response.as_bytes()).unwrap();
}
