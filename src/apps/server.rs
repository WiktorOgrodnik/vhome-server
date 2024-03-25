use tide::Redirect;

use dotenv::dotenv;

use lib::{db_connection_tide, State};
use lib::routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let db = db_connection_tide().await?;
    let mut app = tide::with_state(State {db: db.clone() });

    app.at("/").get(Redirect::new("/home"));
    app.at("/home").get(routes::vlist::all);
    
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
