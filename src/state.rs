use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub secret: SecretWrapper,
}

#[derive(Clone)]
pub struct SecretWrapper(pub String);
