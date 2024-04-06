use serde::{Serialize, Deserialize};
use sqlx::{prelude::FromRow, PgPool};
use strum::EnumString;

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum Participation {
    Guest,
    Member,
    Admin,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Data {
    pub id: i32,
    pub name: String,
}

impl Data {
    pub async fn get(db: &PgPool, interface: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM vgroup WHERE vgroup.id = $1
            ",
            interface
        ).fetch_one(db).await
    }
}
