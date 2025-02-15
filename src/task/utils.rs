use super::Task;

pub fn is_task_exist (tasks: &Vec<Task>, task: &Task) -> bool {
    for t in tasks.iter() {
        if t.name.to_lowercase()== task.name.to_lowercase() {
            return true;
        }
      }

    false
}