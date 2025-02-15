use clap::{CommandFactory, Parser};
use task_manager::cli::{Commands, Cli};
use task_manager::delete_task;
use task_manager::task::{add_task, Task};
use task_manager::board::show_board;


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add(arg)) => {
            let task = Task {
                name: arg.name.clone(),
                status: arg.status.clone(),
                description: arg.description.clone(),
            };

            add_task(task, &arg.file).expect("Failed to add task");
        },
        Some(Commands::Board(arg)) => {
            show_board(&arg.file);
        },
        Some(Commands::Delete(arg)) => {
            delete_task(&arg.name , &arg.file).expect("Failed to delete task");
        },
        None => {
            Cli::command().print_help().unwrap();
        },
    }
}
