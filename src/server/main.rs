use tide::Redirect;
use lib::{db_connection, State};
use lib::routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    
    let db = db_connection().await?;
    let mut app = tide::with_state(State {db: db.clone() });

    app.at("/").get(Redirect::new("/home"));
    app.at("/home").get(routes::vlist::all);
    
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
