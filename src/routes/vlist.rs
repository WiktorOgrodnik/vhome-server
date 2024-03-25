use crate::records::{vlist, RecordShow};

pub async fn all(request: crate::Request) -> tide::Result<tide::Body> {
    let vlists = vlist::Data::all(&request.state().db).await?;

    Ok(tide::Body::from_json(&vlists)?)
}
