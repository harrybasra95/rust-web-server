use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Default, PartialEq)]
pub enum RequestTypes {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
}

impl FromStr for RequestTypes {
    type Err = ();

    fn from_str(s: &str) -> Result<RequestTypes, Self::Err> {
        match s {
            "GET" => Ok(RequestTypes::GET),
            "POST" => Ok(RequestTypes::POST),
            "PUT" => Ok(RequestTypes::PUT),
            "DELETE" => Ok(RequestTypes::DELETE),
            _ => Err(()),
        }
    }
}
