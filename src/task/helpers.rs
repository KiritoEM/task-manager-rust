use super::Task;
use crate::clean_str;

pub fn is_task_exist (tasks: &Vec<Task>, name: &String) -> bool {
    for t in tasks.iter() {
        if clean_str(&t.name)== clean_str(&name) {
            return true;
        }
      }

    false
}

pub fn index_of_task(name: &String, tasks: &Vec<Task>) -> usize {
    tasks.iter().position(|t|  clean_str(&t.name)== clean_str(&name) ).unwrap()
}