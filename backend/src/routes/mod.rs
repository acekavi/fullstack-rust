mod hello_world;
mod ext_text;
mod ext_json;
mod ext_path;
mod query_params;
mod custom_headers;
mod middleware_msg;
mod get_json;

use axum::Extension;
use axum::http::Method;
use axum::{Router, routing::get, routing::post};
use tower_http::cors::{CorsLayer, Any};

use self::get_json::get_json;
use self::hello_world::hello_world;
use self::ext_text::ext_text;
use self::ext_json::ext_json;
use self::ext_path::{ext_path,ext_path_string,ext_path_abs};
use self::query_params::query_params;
use self::custom_headers::custom_headers;
use self::middleware_msg::middleware_msg;

#[derive(Clone)]
pub struct SharedData{
  pub message: String,
}

pub fn create_routes() -> Router<>{
  let shared_data = SharedData{
    message: "Hello Mf!".to_owned()
  };

  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

  Router::new()
  .route("/hello", get(hello_world))
  .route("/text", post(ext_text))
  .route("/json", post(ext_json))
  .route("/user/6", get(ext_path_abs))
  .route("/user/:id", get(ext_path))
  .route("/user/name/:name", get(ext_path_string))
  .route("/", get(query_params))
  .route("/custom_header", get(custom_headers))
  .route("/mddleware_msg", get(middleware_msg))
  .route("/get_json", get(get_json))
  .layer(Extension(shared_data))
  .layer(cors)
}