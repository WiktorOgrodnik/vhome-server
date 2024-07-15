use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use sea_orm::DatabaseConnection;

use crate::{queries::group::get_group, records::user::UserExtension};

pub async fn requires_group(
    Extension(user): Extension<UserExtension>,
    State(db): State<DatabaseConnection>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let group_id = user.group_id.ok_or(StatusCode::UNAUTHORIZED)?;
    let _ = get_group(&db, user.id, group_id).await?;

    Ok(next.run(request).await)
}
