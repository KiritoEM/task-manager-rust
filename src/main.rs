use clap::{CommandFactory, Parser};
use task_manager::cli::{Commands, Cli};
use task_manager::task::{add_task, Task};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add(arg)) => {
            let task = Task {
                name: arg.name.clone(),
                status: arg.status.clone(),
                description: arg.description.clone(),
            };

            add_task(task, &arg.file.clone()).expect("Failed to add task");
        },
        Some(Commands::List) => {
            println!("Affichage de la liste des tâches.");
        },
        None => {
            Cli::command().print_help().unwrap();
        },
    }
}
