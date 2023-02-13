use crate::routes::create_routes;

pub async fn app(){
  let index = create_routes();

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
      .serve(index.into_make_service())
      .await
      .unwrap();
}