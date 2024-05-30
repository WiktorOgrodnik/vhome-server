use crate::records::thermometer;
use crate::session_utils::UserGroupSessionInd;

use crate::records::device::{self, DeviceType};

pub async fn all(request: crate::Request) -> tide::Result<tide::Body> {
    let group: Option<UserGroupSessionInd> = request.session().get("user_group");
    let group_id = group
        .expect("CRITICAL ERROR! Group is not defined for show list request!")
        .group
        .id;

    let devices = device::Data::all(&request.state().db, group_id).await?;

    tide::Body::from_json(&devices)
}

pub async fn get(request: crate::Request) -> tide::Result<tide::Body> {
    let group: Option<UserGroupSessionInd> = request.session().get("user_group");
    let group_id = group
        .expect("CRITICAL ERROR! Group is not defined for show list request!")
        .group
        .id;

    let device_id = request.param("device_id")?.parse()?;

    let device = device::Data::get_guarded(&request.state().db, device_id, group_id).await?;

    match device.dev_t {
        DeviceType::Thermometer => Ok(tide::Body::from_json(
            &thermometer::Thermometer::get(&request.state().db, device_id).await?,
        )?),
        DeviceType::Other => todo!(),
    }
}
