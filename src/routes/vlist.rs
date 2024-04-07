use crate::records::{vgroup::UserGroupSessionInd, vlist};

pub async fn all(request: crate::Request) -> tide::Result<tide::Body> {

    let group: Option<UserGroupSessionInd> = request.session().get("user_group");
    let group_id = group.expect("CRITICAL ERROR! Group is not defined for all lists request!").group.id;

    let vlists = vlist::Data::all(&request.state().db, group_id).await?;

    Ok(tide::Body::from_json(&vlists)?)
}

pub async fn show(request: crate::Request) -> tide::Result<tide::Body> {
 
    let group: Option<UserGroupSessionInd> = request.session().get("user_group");
    let group_id = group.expect("CRITICAL ERROR! Group is not defined for show list request!").group.id;
    
    let list_id = request.param("list_id")?.parse()?;

    let vlists = vlist::Data::get(&request.state().db, &vlist::ShowInterface { id: list_id, group_id }).await?;

    Ok(tide::Body::from_json(&vlists)?)
}
