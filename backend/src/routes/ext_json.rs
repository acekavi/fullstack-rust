use std::ops::Add;

use axum::Json;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct ExtractJson {
  message: String,
}

#[derive(Serialize)]
pub struct JsonResponse{
  message: String,
  uid: i32,
}

pub async fn ext_json(Json(body): Json<ExtractJson>) -> Json<JsonResponse>{
  Json(JsonResponse{message:body.message.add(" from the other side"), uid:89079698})
}