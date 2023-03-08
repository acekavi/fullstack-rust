mod lib;
use dotenvy_macro::dotenv;

use lib::run;

#[tokio::main]
async fn main() {  
    let db_url= dotenv!("DB_URL");
    run(db_url).await
}
