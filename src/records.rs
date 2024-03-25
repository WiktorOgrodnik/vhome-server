pub mod vlist;

use sqlx::{postgres::PgQueryResult, PgPool};

pub trait RecordAdd {
    type AddInterface;

    fn add(db: &PgPool, interface: &Self::AddInterface) -> impl std::future::Future<Output=Result<PgQueryResult, sqlx::Error>> + Send 
        where Self: Sized; 
}

pub trait RecordShow {
    type ShowInterface;

    fn get(db: &PgPool, interface: &Self::ShowInterface) -> impl std::future::Future<Output=Result<Self, sqlx::Error>> + Send
        where Self: Sized;
    fn all(db: &PgPool) -> impl std::future::Future<Output=Result<Vec<Self>, sqlx::Error>> + Send
        where Self: Sized;
}

pub trait RecordDelete {
    type DeleteInterface;

    fn delete(db: &PgPool, interface: &Self::DeleteInterface) -> impl std::future::Future<Output=Result<PgQueryResult, sqlx::Error>> + Send
        where Self: Sized;
}

