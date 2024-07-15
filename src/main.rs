use std::net::SocketAddr;

use dotenv::dotenv;
use lib::router;
use lib::state::{AppState, SecretWrapper};
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();
    let secret = env::var("TIDE_SECRET").unwrap();
    let db = Database::connect(database_url).await.unwrap();

    let app = router::init_router(AppState {
        db,
        secret: SecretWrapper(secret),
    });
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
