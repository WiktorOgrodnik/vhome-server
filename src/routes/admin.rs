pub async fn main(_: crate::Request) -> tide::Result<tide::Body> {
    Ok(tide::Body::from_string("<h1>Admin</h1>".to_owned()))
}
