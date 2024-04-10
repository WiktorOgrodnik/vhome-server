use crate::roles::AuthorizeLevel;
use crate::session_utils::UserGroupSessionInd;

use crate::records::{vlist, vtask};

pub async fn all(request: crate::Request) -> tide::Result {
    use vlist::VResult;

    let group: Option<UserGroupSessionInd> = request.session().get("user_group");
    let group_id = group.expect("CRITICAL ERROR! Group is not defined for all lists request!").group.id;

    let list_result: VResult = request.param("list_id")
        .ok()
        .map_or(VResult::None, |t| VResult::Ok(t.parse::<i32>().expect("list id has to be a number!")))
        .authorize(&request, AuthorizeLevel::Show)
        .await;
    
    Ok(match list_result {
        VResult::Forbidden => tide::Response::new(404),
        list_id => tide::Response::builder(200)
            .body(tide::Body::from_json(&vtask::Data::all(&request.state().db, group_id, list_id.to_some()).await?)?)
            .build()
    })
}

pub async fn show(request: crate::Request) -> tide::Result<tide::Body> {
 
    let group: Option<UserGroupSessionInd> = request.session().get("user_group");
    let group_id = group.expect("CRITICAL ERROR! Group is not defined for show list request!").group.id;
    
    let list_id = request.param("list_id")?.parse()?;

    let vlists = vlist::Data::get_guarded(&request.state().db, &vlist::ShowInterface { id: list_id, group_id }).await?;

    Ok(tide::Body::from_json(&vlists)?)
}
