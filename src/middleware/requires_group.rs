use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use sea_orm::DatabaseConnection;

use crate::{queries::group::get_group_db, records::user::UserExtension};

pub async fn requires_group(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    match user {
        UserExtension::GroupUnselected(_) => Err(StatusCode::UNAUTHORIZED),
        UserExtension::GroupSelected(payload) => {
            let _ = get_group_db(&db, payload.id, payload.group_id).await?;
            Ok(next.run(request).await)
        }
    }
}
