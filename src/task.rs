use std::str::FromStr;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum)]
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
#[serde(rename_all="camelCase")]
pub struct Task {
    name: String,
    status: String,
    description: String
}