use database::run;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {  
    let db_url= dotenv!("DATABASE_URL");
    run(db_url).await
}
