use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::types::RequestData;

pub fn get_req_data(stream: &mut TcpStream) -> Option<RequestData> {
    let buf_reader = BufReader::new(stream);
    let request_str = buf_reader.lines().next();

    if request_str.is_none() {
        return None;
    }

    let request_line: Vec<String> = request_str
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    Some(RequestData {
        method: request_line[0].clone(),
        url: request_line[1].clone(),
    })
}
