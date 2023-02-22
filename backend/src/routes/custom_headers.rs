use axum::http::{HeaderMap, HeaderValue};

pub async fn custom_headers(headers : HeaderMap) -> String{
  let message_value: &HeaderValue = headers.get("datakey").unwrap();
  let message = message_value.to_str().unwrap().to_owned();

  message
}