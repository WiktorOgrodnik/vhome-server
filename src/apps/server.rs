use lib::roles::Roles;
use tide::{Redirect, Response, StatusCode};
use tide::http::Cookie;

use sqlx::postgres::PgPool;

use dotenv::dotenv;

use lib::State;
use lib::routes;
use lib::authentication::AuthorizeRouteExt;

async fn db_connection() -> tide::Result<PgPool> {
    Ok(lib::db_connection().await?)
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

    let db = db_connection().await?;
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
        .authorized(vec![])
        .get(routes::vgroup::get)
        .post(routes::vgroup::set)
        .delete(routes::vgroup::unregister);

    // Get lists
    app.at("/lists")
        .authorized_group(vec![Roles::Member])
        .get(routes::vlist::all);

    app.at("/list/:list_id")
        .authorized_group(vec![Roles::Member])
        .get(routes::vlist::show);
    
    // Get tasks
    app.at("/tasks")
        .authorized_group(vec![Roles::Guest, Roles::Member])
        .get(routes::vtask::all);

    app.at("/tasks/:list_id")
        .authorized_group(vec![Roles::Guest, Roles::Member])
        .get(routes::vtask::all)
        .post(routes::vtask::add);

    app.at("/task/:task_id")
        .authorized_group(vec![Roles::Guest, Roles::Member])
        .get(routes::vtask::show)
        .delete(routes::vtask::delete);

    app.at("/task/completed/:task_id/:value")
        .authorized_group(vec![Roles::Guest, Roles::Member])
        .put(routes::vtask::set_completed);

    app.at("/devices")
        .authorized_group(vec![Roles::Guest, Roles::Member])
        .get(routes::device::all);

    app.at("/device/:device_id")
        .authorized_group(vec![Roles::Guest, Roles::Member])
        .get(routes::device::get);

    // Session management
    app.at("/authenticate").post(routes::authenticate::login);
    app.at("/logout").post(routes::authenticate::logout);
    app.at("/admin").authorized(vec![]).get(routes::admin::main);
    
    // Cookie debug
    app.at("/set").get(insert_cookie);
    app.at("/get").get(get_cookie);
    app.at("/remove").get(remove_cookie);

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
