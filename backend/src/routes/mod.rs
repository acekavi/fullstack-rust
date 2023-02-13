mod hello_world;
mod ext_text;
mod ext_json;

use axum::{Router, routing::get, routing::post};

use self::hello_world::hello_world;
use self::ext_text::ext_text;
use self::ext_json::ext_json;

pub fn create_routes() -> Router<>{
  Router::new().route("/hello", get(hello_world))
  .route("/text", post(ext_text))
  .route("/json", post(ext_json))
}