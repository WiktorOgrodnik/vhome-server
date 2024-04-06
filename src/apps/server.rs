use lib::records::vuser;
use tide::{Middleware, Next, Redirect, Request, Response, Route, StatusCode};
use tide::http::Cookie;

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
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        let user: Option<vuser::Data> = req.session().get("user");
        match user {
            Some(_user) => Ok(next.run(req).await),
            None => Ok(Redirect::new("/home").into()),
        }        
    }
}

async fn insert_cookie(_req: lib::Request) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.insert_cookie(Cookie::new("hello", "world"));
    Ok(res)
}

async fn get_cookie(req: lib::Request) -> tide::Result<String> {
    Ok(format!("hello cookie: {:?}", req.cookie("hello").unwrap()))
}

async fn remove_cookie(_req: lib::Request) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.remove_cookie(Cookie::named("hello"));
    Ok(res)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let db = db_connection_tide().await?;
    let mut app = tide::with_state(State {db: db.clone()});
    
    app.with(tide::log::LogMiddleware::new());

    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        std::env::var("TIDE_SECRET")
            .expect(
                "Please provide a TIDE_SECRET value at \
                    least 32 bytes in order to run this server.",
            )
            .as_bytes(),
    ));

    app.at("/").get(Redirect::new("/home"));
    app.at("/home").get(routes::greet::default);
    
    // Set group
    app.at("/setgroup/:group_id")
        .authorized()
        .get(routes::vgroup::get)
        .post(routes::vgroup::set)
        .delete(routes::vgroup::unregister);

    // Get lists
    app.at("/lists")
        .authorized()
        .get(routes::vlist::list);

    app.at("/list/:list_id")
        .authorized()
        .get(routes::vlist::show);

    // Session management
    app.at("/authenticate").post(routes::authenticate::login);
    app.at("/logout").post(routes::authenticate::logout);
    app.at("/admin").authorized().get(routes::admin::main);
    
    // Cookie debug
    app.at("/set").get(insert_cookie);
    app.at("/get").get(get_cookie);
    app.at("/remove").get(remove_cookie);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
