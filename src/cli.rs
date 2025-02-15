use clap::{arg, Args, Parser, Subcommand};
use crate::TaskStatus;

#[derive(Parser)]
#[command(author, version)]
#[command(author = "KiritoEM", version, about = "\n\nTask manager with Rust")]
pub struct Cli {
    #[clap(subcommand)] 
   pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about="Add a new task")]
    Add(Add),

    #[command(about="Show board of tasks")]
    Board(Board),

    #[command(about="Delete an task")]
    Delete(Delete),
}

#[derive(Args)]
pub struct Add {
    #[arg(short = 'n', long="name", help = "Name of task")]
    pub name: String,

    #[arg(short = 's', long="status", default_value = "todo", help = "Status of task")]
    pub status: TaskStatus,

    #[arg(short = 'f', long="file", help = "Path of task file")]
    pub file: String,


    #[arg(short = 'd', long="description", help = "Description of task")]
    pub description: String,
}

#[derive(Args)]
pub struct Delete {
    #[arg(short = 'n', long="name", help = "Name of task to delete")]
    pub name: String,

    #[arg(short = 'f', long="file", help = "Path of task file")]
    pub file: String,
}

#[derive(Args)]
pub struct Board {
    #[arg(short = 'f', long="file", help = "Path of task file to show")]
    pub file: String,
}

