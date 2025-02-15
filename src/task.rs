use std::str::FromStr;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use crate::{load_tasks_file, save_tasks_file};

#[derive(Debug, Clone, ValueEnum, Deserialize, Serialize)]
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


pub fn add_task(task: Task, path: &String) -> Result<(), std::io::Error> {
    let mut tasks = load_tasks_file(&path).expect("Failed to load tasks");

    tasks.push(task);
    match save_tasks_file((&path).to_string(), &tasks) {
        Ok(_) => {
            println!("Task added successfully!");
            Ok(())
    },
        Err(e) => return Err(e),
    }
}
