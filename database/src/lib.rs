mod routes;
use axum::Router;
use sea_orm::Database;

pub async fn run(db_uri: &str) {
  let database = Database::connect(db_uri).await.unwrap();

  let app: Router = routes::create_routes(database).await;

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}