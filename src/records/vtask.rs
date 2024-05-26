use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, types::chrono, PgPool};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Data {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub vlist_id: i32,
    pub completed_time: Option<chrono::DateTime<chrono::Utc>>, 
}

#[derive(Deserialize)]
pub struct AddInterface {
    pub title: String,
    pub content: String,
}

impl Data {

    async fn all_from_group(db: &PgPool, vgroup_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT vtask.* FROM vtask JOIN vlist ON vlist.id = vtask.vlist_id
                WHERE vlist.group_id = $1
            ",
            vgroup_id,
        ).fetch_all(db).await
    }

    async fn all_from_list(db: &PgPool, vgroup_id: i32, vlist_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT vtask.* FROM vtask JOIN vlist ON vlist.id = vtask.vlist_id
                WHERE vlist.group_id = $1 AND vlist.id = $2 
            ",
            vgroup_id,
            vlist_id,
        ).fetch_all(db).await
    }

    pub async fn all(db: &PgPool, vgroup_id: i32, vlist_id: Option<i32>) -> Result<Vec<Self>, sqlx::Error> {
        match vlist_id {
            Some(vlist_id) => Self::all_from_list(db, vgroup_id, vlist_id).await,
            None => Self::all_from_group(db, vgroup_id).await,
        }
    }

    pub async fn get(db: &PgPool, interface: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(Self,
            "
            SELECT * FROM vtask WHERE vtask.id = $1 
            ",
            interface,
        ).fetch_one(db).await
    }
    
    pub async fn set_completed_guarded(db: &PgPool, vtask_id: i32, value: bool, vgroup_id: i32) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "
            UPDATE vtask t
            SET completed = $1
            FROM vlist l
            WHERE t.vlist_id = l.id AND t.id = $2 AND l.group_id = $3 
            ",
            value,
            vtask_id,
            vgroup_id,
        ).execute(db).await
    }

    pub async fn add(db: &PgPool, vlist_id: i32, interface: AddInterface) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO vtask (
                title,
                content,
                completed,
                vlist_id,
                completed_time )
            VALUES ($1, $2, $3, $4, NULL)
            ",
            interface.title,
            interface.content,
            false,
            vlist_id,
        ).execute(db).await
    }

    pub async fn delete(db: &PgPool, vtask_id: i32) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
        "
        DELETE FROM vtask WHERE id = $1
        ",
        vtask_id
        ).execute(db).await
    }
}
