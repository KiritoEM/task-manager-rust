use std::{process::exit, str::FromStr};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use crate::{load_tasks_file, save_tasks_file};

pub mod utils;

use utils::{index_of_task, is_task_exist};

#[derive(Debug, Clone, ValueEnum, Deserialize, Serialize, PartialEq)]
pub enum TaskStatus {
    TODO,
    INPROGRESS,
    DONE,
}

impl FromStr for TaskStatus {
    type Err = String;

     fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "todo" => Ok(TaskStatus::TODO),
            "inprogress" => Ok(TaskStatus::INPROGRESS),
            "done" => Ok(TaskStatus::DONE),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
#[warn(unused_variables)]
#[serde(rename_all="camelCase")]
pub struct Task {
    pub name: String,
    pub  status: TaskStatus,
    pub  description: String
}


pub fn add_task(task: Task, path: &String) {
    let mut tasks = load_tasks_file(&path);

    if is_task_exist(&tasks, &task.name) {
        println!("\n\nTask already exist");
        exit(1);
    }

    tasks.push(task);
    match save_tasks_file(&path, &tasks) {
        Ok(_) => println!("\n\nTask added successfully"),
        Err(e) => println!("\n\nError: {}", e),
    }
}

pub fn delete_task(task_name: &String, path: &String)  {
    let mut tasks = load_tasks_file(&path);

    if !is_task_exist(&tasks, &task_name) {
        println!("\n\nTask not found");
        exit(1);
    }

    let index = index_of_task(&task_name, &tasks);
    tasks.remove(index);

    match save_tasks_file(&path, &tasks) {
        Ok(_) => println!("\n\nTask removed successfully"),
        Err(e) => println!("\n\nError: {}", e),
    }
}
