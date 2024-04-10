use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, prelude::FromRow, PgPool};

use clap::Args;
use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};

use crate::roles::Roles;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Data {
    pub id: i32,
    pub login: String,
    pub passwd: String,
}

#[derive(serde::Deserialize, Args)]
#[command(arg_required_else_help = true)]
pub struct _AddInterface {
    pub login: String,
    pub passwd: String,
}

pub struct _ShowDeleteInterface {
    pub id: i32,
}

#[derive(serde::Deserialize, Args)]
#[command(arg_required_else_help = true)]
pub struct AddToGroupInterface {
    pub user_id: i32,
    pub group_id: i32,
}

pub type AddInterface = _AddInterface;
pub type ShowInterface = _ShowDeleteInterface;
pub type DeleteInterface = _ShowDeleteInterface;

#[derive(Debug)]
pub enum Error {
    NoUserError,
    BcryptError(BcryptError),
    PasswordIncorrectError,
    DatabaseError(sqlx::Error),
}

impl Data {
    pub async fn passwd_verify(db: &PgPool, interface: &AddInterface) -> Result<i32, Error> {
        use Error::{BcryptError, PasswordIncorrectError, DatabaseError, NoUserError};    
    
        let user = sqlx::query!(
            "
            SELECT vuser.id, vuser.passwd FROM vuser
                WHERE vuser.login = $1
            ",
            interface.login,
        ).fetch_optional(db).await.map_err(|err| DatabaseError(err))?;
        
        match user {
            None => Err(NoUserError),
            Some(user) => {
                let hashed = verify(interface.passwd.clone(), &user.passwd).map_err(|err| BcryptError(err))?;

                if hashed { Ok(user.id) } else { Err(PasswordIncorrectError) }
            },
        }
    }

    pub async fn add(db: &PgPool, interface: &AddInterface) -> Result<PgQueryResult, Error> {
        use Error::{BcryptError, DatabaseError};

        let passwd = hash(interface.passwd.clone(), DEFAULT_COST).map_err(|err| BcryptError(err))?;

        sqlx::query!(
            "
            INSERT INTO vuser (login, passwd) VALUES ($1, $2)
            ",
            interface.login,
            passwd,
        ).execute(db).await.map_err(|err| DatabaseError(err))
    }

    pub async fn add_to_group(db: &PgPool, interface: &AddToGroupInterface) -> Result<PgQueryResult, Error> {   
        use Error::DatabaseError;

        sqlx::query!(
            "
            INSERT INTO vuser_groups (user_id, group_id) VALUES ($1, $2)
            ",
            interface.user_id,
            interface.group_id,
        ).execute(db).await.map_err(|err| DatabaseError(err))
    }

    pub async fn all(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM vuser
            "
        ).fetch_all(db).await
    }

    pub async fn get(db: &PgPool, interface: &ShowInterface) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM vuser
            WHERE vuser.id = $1
            ",
            interface.id
        ).fetch_one(db).await
    }

    pub async fn delete(db: &PgPool, interface: &DeleteInterface) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "
            DELETE FROM vuser WHERE id = $1
            ",
            interface.id
        ).execute(db).await
    }

    pub async fn get_group_participation(&self, db: &PgPool, interface: i32) -> Result<Vec<Roles>, Error> {
        use Error::DatabaseError;
        
        #[derive(FromRow, Deserialize)]
        struct TempName {
            name: String,
        }

        Ok(sqlx::query_as!(TempName,
            "
            SELECT participation.name FROM vuser_groups
                JOIN participation ON participation.id = vuser_groups.participation_id
            WHERE vuser_groups.user_id = $1 AND vuser_groups.group_id = $2
            ",
            interface,
            self.id,
        ).fetch_all(db)
            .await
            .map_err(|e| DatabaseError(e))?
            .iter()
            .map(|t| Roles::from_str(&t.name).unwrap_or(Roles::Guest))
            .collect::<Vec<Roles>>())
    }
}

