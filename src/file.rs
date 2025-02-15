use std::fs::File;
use std::io::{Read, Error};
use crate::parse_str_to_json;
use crate::Task;

pub fn load_tasks_file(path: &String) -> Result<Vec<Task>, Error>{
    let mut file = File::open(path).expect("No such file in path {path}, please verify the path");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read file");

    let tasks: Vec<Task> = match parse_str_to_json(&contents) {
        Ok(tasks) => tasks,
        Err(e) => return Err(e),
        
    };

    Ok(tasks)
}

pub fn save_tasks_file(path: String, tasks: &Vec<Task>) -> Result<(), Error> {
    let file = File::create(path).expect("Unable to create or read file");
    
    serde_json::to_writer(file,&tasks).expect("Unable to write file");

    Ok(())
}