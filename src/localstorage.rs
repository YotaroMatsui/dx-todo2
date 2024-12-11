use gloo_storage::{LocalStorage, Storage};

use crate::Task;

const LOCAL_STORAGE_KEY: &str = "todo_tasks";

pub(crate) fn load_tasks_from_local_storage() -> Vec<Task> {
    LocalStorage::get(LOCAL_STORAGE_KEY).unwrap_or_default()
}

pub(crate) fn save_task_to_local_storage(tasks: Vec<Task>) {
    LocalStorage::set(LOCAL_STORAGE_KEY, tasks).expect("failed to save tasks to local storage");
}
