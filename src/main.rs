use clap::{CommandFactory, Parser};
use task_manager::cli::{Commands, Cli};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add(arg)) => {
            println!("Ajout d'une tâche : {}", arg.name);
            println!("Statut : {:?}", arg.status);
        },
        Some(Commands::List) => {
            println!("Affichage de la liste des tâches.");
        },
        Some(Commands::Done) => {
            println!("Marquage d'une tâche comme terminée.");
        },
        None => {
            Cli::command().print_help().unwrap();
        },
    }
}
