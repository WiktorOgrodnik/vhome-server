use clap::{Args, Parser, Subcommand};

use crate::records::vlist;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    MakeMigrations,
    Add(AddArgs),
    Show(ShowArgs),
    Delete(DeleteArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct AddArgs {
    #[command(subcommand)]
    pub command: Option<AddCommands>,
}

#[derive(Subcommand)]
pub enum AddCommands {
    VList(vlist::AddInterface)
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct DeleteArgs {
    #[command(subcommand)]
    pub command: Option<DeleteCommands>,
}

#[derive(Subcommand)]
pub enum DeleteCommands {
    VList(vlist::DeleteInterface)
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct ShowArgs {
    #[arg(long, action)]
    pub all: bool,

    #[command(subcommand)]
    pub command: Option<ShowCommands>,
}

#[derive(Subcommand)]
pub enum ShowCommands {
    VList(vlist::ShowInterface)
}
