use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Queryparams {
  message : String,
  id : i32
}

pub async fn query_params(Query(query): Query<Queryparams>) -> Json<Queryparams>{
  Json(query)
}