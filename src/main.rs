use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/home").get(default);
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn default(_: Request<()>) -> tide::Result {
    Ok("Hello\n".into())
}
