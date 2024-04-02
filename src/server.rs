use tide::{Middleware, Redirect, Request, Route, Next};

use dotenv::dotenv;

use lib::{db_connection_tide, State};
use lib::routes;

pub trait AuthorizeRouteExt {
    fn authorized(&mut self) -> &mut Self;
}

impl<'a, State> AuthorizeRouteExt for Route<'a, State>
    where State: Clone + Send + Sync + 'static {
    fn authorized(&mut self) -> &mut Self {
        self.with(MustAuthenticateMiddleWare {})
    }
}

struct MustAuthenticateMiddleWare;

#[tide::utils::async_trait]
impl<State> Middleware<State> for MustAuthenticateMiddleWare
    where State: Clone + Send + Sync + 'static {
    async fn handle(&self, _: Request<State>, _: Next<'_, State>) -> tide::Result {
        Ok(Redirect::new("/home").into())
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let db = db_connection_tide().await?;
    let mut app = tide::with_state(State {db: db.clone()});

    app.at("/").get(Redirect::new("/home"));
    app.at("/home").get(routes::vlist::all);
    app.at("/authenticate").post(routes::authenticate::login);

    app.at("/admin").authorized().get(routes::admin::main);
    
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
