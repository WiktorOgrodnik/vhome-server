use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;

use crate::records::user::UserExtension;
use crate::state::SecretWrapper;
use crate::{
    queries::device as queries,
    records::device::{InsertDevice, ResponseDeviceToken},
};

pub async fn add_device(
    Extension(user): Extension<UserExtension>,
    State(secret): State<SecretWrapper>,
    State(db): State<DatabaseConnection>,
    Json(device): Json<InsertDevice>,
) -> Result<(StatusCode, Json<ResponseDeviceToken>), StatusCode> {
    let device: ResponseDeviceToken =
        queries::create_device(&db, device, secret.0, user.id, user.group_id.unwrap())
            .await?
            .into();

    Ok((StatusCode::CREATED, Json(device)))
}
