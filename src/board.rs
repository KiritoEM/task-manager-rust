use std::process::exit;

use crate::{load_tasks_file, TaskStatus};
use prettytable::{row, Table};

pub fn show_board(path: &String)  {
    let tasks =  load_tasks_file(&path);

    if tasks.len() == 0 {
        println!("\n\nNo tasks found, please add task to {path}");
        exit(1)
    }

    let mut task_board = Table::new();

    task_board.add_row(row!["TODO", "IN-PORGRESS", "DONE"]);

    let mut todo_col : Vec<String> = vec![];
    let mut in_progress_col : Vec<String> = vec![];
    let mut done_col : Vec<String> = vec![];

    for t in tasks.iter() {
       match t.status {
        TaskStatus::TODO  => todo_col.push(t.name.to_string()),
        TaskStatus::INPROGRESS  => in_progress_col.push(t.name.to_string()),
        TaskStatus::DONE  => done_col.push(t.name.to_string()),
       }
    }
    
    task_board.add_row(
           row![
               todo_col.join("\n\n"),
               in_progress_col.join("\n\n"),
               done_col.join("\n\n"),
           ]
    );
  

    task_board.printstd();
}