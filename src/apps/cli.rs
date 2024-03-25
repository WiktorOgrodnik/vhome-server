use async_std::{eprintln, println};
use dotenv::dotenv;
use clap::Parser;

use lib::{db_connection_cli, interface::cli::{AddCommands, Cli, Commands, DeleteCommands, ShowCommands}, records::{vlist::{self, Data}, RecordAdd, RecordDelete, RecordShow}};
use sqlx::{migrate::MigrateError, PgPool};

async fn make_migrations(db: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(db).await
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let db = db_connection_cli().await?;
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::MakeMigrations) => { make_migrations(db).await?; },
        Some(Commands::Add(obj)) => {
            let add_command = &obj.command;
            // To do <- do it with macro
            let rows_affected = match add_command {
                Some(AddCommands::VList(t)) => { vlist::Data::add(db, t).await?.rows_affected() },
                None => { 0 }
            };

            eprintln!("Operation completed. Rows affected: {}", rows_affected).await;
        },
        Some(Commands::Delete(obj)) => {
            let delete_command = &obj.command;
            let rows_affected = match delete_command {
                Some(DeleteCommands::VList(t)) => { vlist::Data::delete(db, t).await?.rows_affected() },
                None => { 0 },
            };

            eprintln!("Operation completed. Rows affected: {}", rows_affected).await;
        },
        Some(Commands::Show(obj)) => {
            let show_command = &obj.command;
            let show_all = &obj.all;

            match show_all {
                true => {
                    let rows = match show_command {
                        Some(ShowCommands::VList(_)) => { vlist::Data::all(db).await? },
                        None => { vec![] }
                    };
                    
                    for row in rows.iter() {
                        println!("id: {}, name: {}", row.id, row.name).await;
                    }

                    /* rows.iter().for_each(async move |row| {
                        println!("id: {}, name: {}", row.id, row.name).await;
                    }); */
                },
                false => {
                    let row = match show_command {
                        Some(ShowCommands::VList(t)) => {vlist::Data::get(db, t).await? },
                        None => { Data { id: 0, name: "ee".to_owned() } },
                    };

                    println!("id: {}, name: {}", row.id, row.name).await;
                },
            }
        }
        None => {}
    }

    Ok(())
}
