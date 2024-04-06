use dotenv::dotenv;
use lib::{db_connection, records::vuser};
use sqlx::{postgres::PgQueryResult, PgPool};

async fn add_user(db: &PgPool, user_args: &vuser::AddInterface) -> Result<PgQueryResult, vuser::Error> {
    vuser::Data::add(db, user_args).await
}

async fn add_user_to_group(db: &PgPool, user_group: &vuser::AddToGroupInterface) -> Result<PgQueryResult, vuser::Error> {
    vuser::Data::add_to_group(db, user_group).await
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    AddUser(vuser::AddInterface),
    AddUserToGroup(vuser::AddToGroupInterface),
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let db = db_connection().await?;
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::AddUser(args)) => { add_user(&db, args).await.unwrap(); },
        Some(Commands::AddUserToGroup(args)) => { add_user_to_group(&db, args).await.unwrap(); },
        None => {}
    }

    Ok(())
}
