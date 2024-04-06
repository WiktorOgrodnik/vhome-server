use core::time;
use std::env;

use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgPool};

pub type Request = tide::Request<State>;

#[derive(Clone)]
pub struct State {
    pub db: PgPool,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    message: String,
}

pub async fn db_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap();
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .max_lifetime(time::Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub async fn db_connection_tide() -> tide::Result<PgPool> {
    Ok(db_connection().await?)
}

pub mod records;

pub mod routes {
    pub mod vlist;
    pub mod admin;
    pub mod authenticate;
    pub mod greet;
    pub mod vgroup;
}
