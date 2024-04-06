use tide::{Response, StatusCode};
use crate::records::vuser;

pub async fn login(mut req: crate::Request) -> tide::Result<String> {
    let creds: vuser::AddInterface = req.body_json().await?;
    let id_opt = vuser::Data::passwd_verify(&req.state().db, &creds.into()).await;

    match id_opt {
        Ok(id) => {
            let show_interface = vuser::ShowInterface { id };
            let user = vuser::Data::get(&req.state().db, &show_interface).await?;
            req.session_mut().insert("user", user)?;

            Ok(format!("Logged user: {}", id).to_owned())
        },
        Err(_) => Ok(format!("User can not be logged in!").to_owned())
    }
}

pub async fn logout(mut request: crate::Request) -> tide::Result {
    request.session_mut().destroy();
    Ok(Response::new(StatusCode::Ok))
}
