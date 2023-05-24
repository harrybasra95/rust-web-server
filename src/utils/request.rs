use std::{
    io::Error,
    io::{BufRead, BufReader},
    net::TcpStream,
    result::Result,
};

use crate::{types::RequestData, utils::request};

pub fn get_req_data(stream: &mut TcpStream) -> Option<RequestData> {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.len() == 0 {
        return None;
    }

    let (method, url, query_params) = get_method_url_query_params(http_request.get(0)).unwrap();

    let mut headers: Vec<(String, String)> = Vec::new();
    for line in http_request {
        if line.contains("{") {
            break;
        }
        if line.contains(":") {
            let mut line = line.splitn(2, ":");
            headers.push((line.next()?.to_string(), line.next()?.to_string()))
        }
    }

    Some(RequestData {
        method,
        url,
        query_params,
    })
}

fn get_method_url_query_params(
    req_url_data: Option<&String>,
) -> Option<(String, String, Vec<(String, String)>)> {
    req_url_data.and_then(|url_data| {
        let mut url_parts = url_data.splitn(3, ' ');
        let req_method = url_parts.next()?.to_string();
        let request_url_and_query_params = url_parts.next()?;
        let mut url_parts = request_url_and_query_params.splitn(2, "?");
        let req_url = url_parts.next()?.to_string();
        let query_params = url_parts
            .next()?
            .split('&')
            .filter_map(|param| {
                let mut parts = param.splitn(2, "=");
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
            .collect();
        Some((req_method, req_url, query_params))
    })
}
