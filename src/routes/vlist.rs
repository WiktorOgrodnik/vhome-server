use crate::records::vlist;

pub async fn all(request: crate::Request) -> tide::Result<tide::Body> {
    let vlists = vlist::Data::all(&request.state().db).await?;

    Ok(tide::Body::from_json(&vlists)?)
}

pub async fn list(_request: crate::Request) -> tide::Result<tide::Body> {
    todo!();
}

pub async fn show(_request: crate::Request) -> tide::Result<tide::Body> {
    todo!();
}
