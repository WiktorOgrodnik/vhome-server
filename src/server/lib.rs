use libgrouplist::{db_connection, DbPool};

pub type Request = tide::Request<State>;

#[derive(Clone)]
pub struct State {
    pub db: DbPool,
}

pub async fn db_connection_stub() -> tide::Result {
    Ok(db_connection().await.into())
}

pub mod routes {
    pub mod vlist;
}
