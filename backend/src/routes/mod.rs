mod hello_world;
mod ext_text;
mod ext_json;
mod ext_path;
mod query_params;

use axum::{Router, routing::get, routing::post};

use self::hello_world::hello_world;
use self::ext_text::ext_text;
use self::ext_json::ext_json;
use self::ext_path::{ext_path,ext_path_string,ext_path_abs};
use self::query_params::query_params;

pub fn create_routes() -> Router<>{
  Router::new().route("/hello", get(hello_world))
  .route("/text", post(ext_text))
  .route("/json", post(ext_json))
  .route("/user/6", get(ext_path_abs))
  .route("/user/:id", get(ext_path))
  .route("/user/name/:name", get(ext_path_string))
  .route("/", get(query_params))
}