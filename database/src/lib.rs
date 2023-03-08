use sea_orm::Database;

pub async fn run(db_uri: &str){
  let database = Database::connect(db_uri).await;
}