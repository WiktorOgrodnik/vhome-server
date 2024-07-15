use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use lib::router;
use lib::state::{AppState, SecretWrapper};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = dotenv!("DATABASE_URL").to_owned();
    let secret = dotenv!("SECRET").to_owned();
    let db = Database::connect(database_url).await.unwrap();

    let app = router::init_router(AppState {
        db,
        secret: SecretWrapper(secret),
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
