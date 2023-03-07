use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonData{
  message : String,
  account : i32,
  username : String
}

pub async fn get_json() -> Json<JsonData>{
  let data = JsonData{
    message: "Hello from the other side!".to_owned(),
    account: 96987998,
    username: "Adele".to_owned(),
  };

  Json(data)
}