use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, PgPool};

use crate::authentication::authorize;
use crate::roles::AuthorizeLevel;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Data {
    pub id: i32,
    #[sqlx(default)]
    pub group_id: i32,
    pub name: String,
}

pub struct _AddInterface {
    pub name: String
}

pub struct _ShowDeleteInterface {
    pub id: i32,
    pub group_id: i32,
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

    pub async fn all(db: &PgPool, interface: i32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM vlist WHERE vlist.group_id = $1
            ",
            interface,
        ).fetch_all(db).await
    }

    pub async fn get(db: &PgPool, list_id: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM vlist WHERE vlist.id = $1
            ",
            list_id,
        ).fetch_one(db).await
    }

    pub async fn get_guarded(db: &PgPool, interface: &ShowInterface) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM vlist WHERE vlist.id = $1 AND vlist.group_id = $2
            ",
            interface.id,
            interface.group_id,
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


pub enum VResult {
    Ok(i32),
    Forbidden,
    NotFound,
    None,
}

impl VResult {
    pub async fn authorize(self, request: &crate::Request, level: AuthorizeLevel) -> Self {

        match self {
            Self::Ok(id) => {
                let group_id = Data::get(&request.state().db, id).await;

                match group_id {
                    Ok(querry) => if authorize(request, level, Some(querry.group_id)).await { Self::Ok(id) } else { Self::Forbidden },
                    Err(_) => Self::NotFound,
                }

            },
            Self::Forbidden => Self::Forbidden,
            Self::NotFound => Self::NotFound,
            Self::None => Self::None,
        }
    }

    pub fn to_some(self) -> Option<i32> {
        match self {
            VResult::Ok(id) => Some(id),
            _   => None,
        }
    }
}
