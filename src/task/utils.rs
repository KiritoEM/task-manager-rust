use super::Task;
use crate::clean_str;

pub fn is_task_exist(tasks: &Vec<Task>, name: &String) -> bool {
    let cleaned_name = clean_str(name);
    tasks.iter().any(|t| clean_str(&t.name) == cleaned_name)
}


pub fn index_of_task(name: &String, tasks: &Vec<Task>) -> usize {
    let cleaned_name = clean_str(name);
    tasks.iter().position(|t|  clean_str(&t.name)== cleaned_name ).unwrap()
}