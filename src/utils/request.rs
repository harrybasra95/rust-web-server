use std::{collections::HashMap, io::Read, net::TcpStream};

use crate::types::RequestData;

pub fn get_req_data(stream: &mut TcpStream) -> Option<RequestData> {
    let mut buffer = [0; 512];

    let read_result = stream.read(&mut buffer);

    if read_result.is_err() {
        return None;
    }
    let bytes_read = read_result.unwrap();

    if bytes_read == 0 {
        return None;
    }

    let http_request = String::from_utf8_lossy(&buffer[..]);
    let mut lines = http_request.trim().split("\r\n");

    let first_line = lines.next()?;

    let (method, url, query_params) = get_method_url_query_params(first_line)?;

    let mut has_body_started = false;
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut body: HashMap<String, String> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            has_body_started = true;
            continue;
        }
        if has_body_started {
            get_body_fields(line, &mut body);
            break;
        }
        if line.contains(":") {
            get_headers(line, &mut headers);
        }
    }

    Some(RequestData {
        method,
        url,
        query_params,
        headers,
        body,
    })
}

fn get_method_url_query_params(
    url_data: &str,
) -> Option<(String, String, HashMap<String, String>)> {
    let mut url_parts = url_data.splitn(3, ' ');
    let req_method = url_parts.next()?.to_string();
    let request_url_and_query_params = url_parts.next()?;
    let mut url_parts = request_url_and_query_params.splitn(2, "?");
    let req_url = url_parts.next()?.to_string();
    let mut query_params: HashMap<String, String> = HashMap::new();
    match url_parts.next() {
        Some(url_parts) => url_parts.split('&').for_each(|param| {
            let mut parts = param.splitn(2, "=");
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                query_params.insert(key.to_string(), value.to_string());
            }
        }),
        None => (),
    };
    Some((req_method, req_url, query_params))
}

fn get_body_fields(line: &str, body: &mut HashMap<String, String>) {
    line.trim()
        .replace('{', "")
        .replace("}", "")
        .trim()
        .split(",")
        .for_each(|s| {
            let s = s.trim().replace("\"", "");
            let mut s = s.splitn(2, ":");
            if let (Some(key), Some(value)) = (s.next(), s.next()) {
                body.insert(
                    key.to_string(),
                    value
                        .trim_matches(|c: char| c == '\0' || c.is_whitespace())
                        .to_string(),
                );
            }
        });
}

fn get_headers(line: &str, headers: &mut HashMap<String, String>) {
    let mut line = line.splitn(2, ":");
    if let (Some(key), Some(value)) = (line.next(), line.next()) {
        headers.insert(key.trim().to_string(), value.trim().to_string());
    }
}
