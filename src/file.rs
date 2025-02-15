use std::fs::File;
use std::io::Read;
use std::process::exit;
use crate::parse_str_to_json;
use crate::Task;

pub fn load_tasks_file(path: &String) -> Vec<Task>{
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("\nNo such file in path {path}, please verify the path.");
            exit(1)
        }
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read file");

    if contents.is_empty() {
        return vec![];
    }

    let tasks: Vec<Task> = parse_str_to_json(&contents).expect("Failed to parse JSON");

    tasks
}

pub fn save_tasks_file(path: &String, tasks: &Vec<Task>) -> Result<() , std::io::Error>{
    let file = File::create(path).expect("Unable to create or read file");
    serde_json::to_writer(file,&tasks).expect("Unable to write file");

    Ok(())
}