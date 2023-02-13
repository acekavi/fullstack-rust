mod router;
pub mod routes;

use router::app;

#[tokio::main]
async fn main() {
    app().await
}
