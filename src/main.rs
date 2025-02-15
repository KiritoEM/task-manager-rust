use clap::{CommandFactory, Parser};
use task_manager::cli::{Commands, Cli};
use task_manager::file::load_tasks_file;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add(arg)) => {
            println!("Ajout d'une tâche : {}", arg.name);
            println!("Statut : {:?}", arg.status);

            match load_tasks_file(arg.file.clone()) {
                Ok(contents) => {
                    println!("File content : {:?}", contents);
                },
                Err(e) => {
                    println!("Error : {}", e);
                }
            }
        },
        Some(Commands::List) => {
            println!("Affichage de la liste des tâches.");
        },
        None => {
            Cli::command().print_help().unwrap();
        },
    }
}
