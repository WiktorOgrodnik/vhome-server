#[derive(serde::Deserialize)]
struct Creditionals {
    login: String,
    passwd: String,
} 

pub async fn login(mut req: crate::Request) -> tide::Result<String> {
    let creds: Creditionals = req.body_json().await?; 
    let deb = format!("{}/{}", creds.login, creds.passwd);

    Ok(deb)
}
