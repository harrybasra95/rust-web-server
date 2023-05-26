pub mod request;
pub mod response;

pub fn missing_field_err(field_name: &str) -> String {
    format!("{field_name} is required")
}
