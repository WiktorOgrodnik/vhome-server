use axum::{extract::State, http::StatusCode, Json};
use bcrypt::hash;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

use crate::{
    database::vuser,
    queries::user as queries,
    records::user::{RequestUser, ResponseUser},
};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<(StatusCode, Json<ResponseUser>), StatusCode> {
    let user_exists = queries::find_by_username(&db, request_user.username.clone())
        .await
        .is_ok();

    if user_exists {
        return Err(StatusCode::BAD_REQUEST);
    }

    let active_user = vuser::ActiveModel {
        login: Set(request_user.username),
        passwd: Set(hash(request_user.password, 4).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?),
        created_at: Set(Utc::now().into()),
        ..Default::default()
    };

    let user = active_user
        .save(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into_model()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ResponseUser {
        id: user.id,
        username: user.login,
        created_at: user.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
