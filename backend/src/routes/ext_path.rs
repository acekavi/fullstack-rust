use axum::extract::Path;

pub async fn ext_path(Path(id): Path<i32>) -> String{
  id.to_string()
}

pub async fn ext_path_string(Path(name): Path<String>) -> String{
  name
}

pub async fn ext_path_abs() -> String{
  "Suck my dick".to_owned()
}