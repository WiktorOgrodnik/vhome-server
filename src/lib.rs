use core::time;
use std::{env, sync::OnceLock};

use sqlx::postgres::{PgPoolOptions, PgPool};

pub type Request = tide::Request<State>;

pub static PGPOOL: OnceLock<PgPool> = OnceLock::new();

#[derive(Clone)]
pub struct State {
    pub db: PgPool,
}

async fn db_connection() -> Result<PgPool, sqlx::Error> {
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

pub async fn db_connection_cli() -> Result<&'static PgPool, sqlx::Error> {
    if PGPOOL.get().is_some() {
        Ok(PGPOOL.get().unwrap())
    } else if let Ok(db) = db_connection().await {
        PGPOOL.set(db).unwrap();
        Ok(PGPOOL.get().unwrap())
    } else {
        Err(sqlx::Error::PoolClosed)
    }
}

pub mod records;

pub mod interface {
    pub mod cli;
}

pub mod routes {
    pub mod vlist;
}
