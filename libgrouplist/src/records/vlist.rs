use serde::{Deserialize, Serialize};
use sqlx::{Postgres, postgres::PgArguments};

// type Query = sqlx::query::Query<'static, Postgres, PgArguments>;
type QueryAs<T> = sqlx::query::QueryAs<'static, Postgres, T, PgArguments>;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct VList {
    pub id: i32,
    pub name: String,
}

impl VList {
    pub fn all() -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM list")
    }
}

