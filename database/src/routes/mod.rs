mod hello_world;

use axum::{Router, routing::get, Extension};

use hello_world::hello_world;
use sea_orm::DatabaseConnection;

pub async fn create_routes(database : DatabaseConnection) -> Router<>{
  Router::new()
  .route("/hello", get(hello_world))
  .layer(Extension(database))
}