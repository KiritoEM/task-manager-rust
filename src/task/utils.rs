use super::Task;

pub fn is_task_exist (tasks: &Vec<Task>, name: &String) -> bool {
    for t in tasks.iter() {
        if t.name.to_lowercase()== name.to_lowercase() {
            return true;
        }
      }

    false
}

pub fn index_of_task(name: &String, tasks: &Vec<Task>) -> usize {
    tasks.iter().position(|t| t.name.to_lowercase()== name.to_lowercase()).unwrap()
}