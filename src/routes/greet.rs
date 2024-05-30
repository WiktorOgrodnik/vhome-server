#[derive(serde::Serialize)]
struct Message {
    content: String,
}

pub async fn default(_: crate::Request) -> tide::Result<tide::Body> {
    let content = "Welcome to grouplist API!".to_owned();
    let message = Message { content };

    tide::Body::from_json(&message)
}
