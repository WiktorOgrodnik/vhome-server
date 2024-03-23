use libgrouplist::records::vlist::VList;

pub async fn all(request: crate::Request) -> tide::Result<tide::Body> {
    let vlists = VList::all().fetch_all(&request.state().db).await?;

    // let vlists = vec![
        // VList { id: 1, name: "Lista zakupów".to_owned() }
    // ];

    Ok(tide::Body::from_json(&vlists)?)
}
