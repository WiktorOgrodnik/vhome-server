use core::time;
use std::env;

use sqlx::postgres::{PgPoolOptions, PgPool};

pub type DbPool = PgPool;

pub async fn db_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap();
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .max_lifetime(time::Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

pub mod records {
    pub mod vlist;
}
