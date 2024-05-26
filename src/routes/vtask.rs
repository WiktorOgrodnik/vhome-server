use tide::StatusCode;

use crate::roles::AuthorizeLevel;
use crate::session_utils::{self, UserGroupSessionInd};

use crate::records::{vlist, vtask};

pub async fn all(request: crate::Request) -> tide::Result {
    use vlist::VResult;

    let group_id = session_utils::get_group(&request)
        .await
        .expect("CRITICAL ERROR! Group is not defined for all lists request!")
        .id;

    let list_result: VResult = request.param("list_id")
        .ok()
        .map_or(VResult::None, |t| VResult::Ok(t.parse::<i32>().expect("list id has to be a number!")))
        .authorize(&request, AuthorizeLevel::Show)
        .await;
    
    Ok(match list_result {
        VResult::Forbidden |
        VResult::NotFound => tide::Response::new(404),
        list_id => tide::Response::builder(200)
            .body(tide::Body::from_json(&vtask::Data::all(&request.state().db, group_id, list_id.ok()).await?)?)
            .header("content-type", "application/json;charset=UTF-8")
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

pub async fn set_completed(request: crate::Request) -> tide::Result {
    let group: Option<UserGroupSessionInd> = request.session().get("user_group");
    let group_id = group.expect("CRITICAL ERROR! Group is not defined for show list request!").group.id;

    let task_id: i32 = request.param("task_id")?.parse()?;
    let value: bool = request.param("value")?.parse()?;

    let _ = vtask::Data::set_completed_guarded(
        &request.state().db,
        task_id,
        value,
        group_id).await?;

    Ok(tide::Response::new(StatusCode::Ok))
}

pub async fn add(mut request: crate::Request) -> tide::Result {
    let group: Option<UserGroupSessionInd> = request.session().get("user_group");
    let group_id = group.expect("CRITICAL ERROR! Group is not defined for show list request!").group.id;

    let task_set_id: i32 = request.param("list_id")?.parse()?;
    let payload: vtask::AddInterface = request.body_json().await?;

    let authorization: bool = vlist::Data::has_permission(
        &request.state().db,
        group_id,
        task_set_id).await;

    if !authorization {
        return Ok(tide::Response::new(StatusCode::Forbidden));
    }

    let _ = vtask::Data::add(
        &request.state().db,
        task_set_id,
        payload,
    ).await?;

    Ok(tide::Response::new(StatusCode::Ok))
}

pub async fn delete(request: crate::Request) -> tide::Result {
    let task_id: i32 = request.param("task_id")?.parse()?;
    
    // Authorization!!!

    let _ = vtask::Data::delete(
        &request.state().db,
        task_id,
    ).await?;

    Ok(tide::Response::new(StatusCode::Ok))
}
