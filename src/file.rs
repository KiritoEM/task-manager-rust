use std::fs::File;
use std::io::{Read, Error};
use crate::{parse_str_to_json, Task};

pub fn load_tasks_file(path: String) -> Result<Vec<Task>, Error>{
    let mut file = File::open(path).expect("No such file, please verify the path");
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    
    let tasks: Vec<Task> = match parse_str_to_json(&contents) {
        Ok(tasks) => tasks,
        Err(e) => return Err(e),
        
    };

    Ok(tasks)
}