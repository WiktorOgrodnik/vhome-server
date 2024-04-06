use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Data {
    pub id: i32,
    pub group_id: i32,
    pub name: String,
}

pub struct _AddInterface {
    pub name: String
}

pub struct _ShowDeleteInterface {
    pub id: i32,
}

pub type AddInterface = _AddInterface;
pub type ShowInterface = _ShowDeleteInterface;
pub type DeleteInterface = _ShowDeleteInterface;

impl Data {

    pub async fn add(db: &PgPool, interface: &AddInterface) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO vlist (name) VALUES ($1)
            ",
            interface.name
        ).execute(db).await
    }

    pub async fn all(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM vlist
            "
        ).fetch_all(db).await
    }

    pub async fn get(db: &PgPool, interface: &ShowInterface) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM vlist
            WHERE vlist.id = $1
            ",
            interface.id
        ).fetch_one(db).await
    }

    pub async fn delete(db: &PgPool, interface: &DeleteInterface) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM vlist WHERE id = $1
            ",
            interface.id
        ).execute(db).await
    }
}

