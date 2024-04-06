use serde::{Serialize, Deserialize};
use tide::{Response, StatusCode};

use crate::{records::{vgroup::{self, Participation}, vuser}, Message};

#[derive(Debug, Serialize, Deserialize)]
struct UserGroupSessionInd {
    group: vgroup::Data,
    roles: Vec<vgroup::Participation>,
}


pub async fn set_for_user(mut request: crate::Request, user: vuser::Data) -> tide::Result {
    let group_id: i32 = request.param("group_id")?.parse()?;
    
    let group = vgroup::Data::get(&request.state().db, group_id).await?;
    let roles = user.get_group_participation(
        &request.state().db, 
        group_id,
    ).await.unwrap(); //to-do make vuser::Error error

   /*  .map_err(|err| {
        use vuser::Error;

        tide::Error::new(StatusCode::InternalServerError, match err {
            Error::DatabaseError(err) => err,
            _ => Error
        }
    }))?;
 */
    let status = if roles.iter()
        .filter(|&elt| matches!(elt, Participation::Member))
        .count() > 0 {
        request.session_mut().insert(
            "user_group",
            UserGroupSessionInd {
                group, roles,
            },
        )?;

        StatusCode::Ok
    } else {
        StatusCode::Forbidden
    };

    Ok(Response::new(status))
}

pub async fn set(request: crate::Request) -> tide::Result {
    let user: Option<vuser::Data> = request.session().get("user");
    
    if let Some(user) = user {
        set_for_user(request, user).await 
    } else {
        Ok(Response::new(StatusCode::Forbidden))
    }
}

pub async fn get(request: crate::Request) -> tide::Result<tide::Body> {
    let user: Option<vuser::Data> = request.session().get("user");
    let group_roles: Option<UserGroupSessionInd> = request.session().get("user_group");

    match (user, group_roles) {
        (Some(_), Some(roles)) => Ok(tide::Body::from_json(&Message { message : format!("Selected group: {:?}", roles) })?),
        (Some(user), None) => Ok(tide::Body::from_json(&Message { message: format!("Group not selected for this user ({:?})", user.login) })?),
        _ => Ok(tide::Body::from_json(&Message { message: "User not logged in.".to_owned() })?),
    }
}

pub async fn unregister(mut request: crate::Request) -> tide::Result {
    request.session_mut().remove("user_group");

    Ok(Response::new(StatusCode::Ok))
}
