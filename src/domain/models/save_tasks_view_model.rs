use serde::{Deserialize, Serialize};

use super::task::Task;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaveTasksViewModel {
    pub tasks: Vec<Task>,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskDbModel {
    pub tasks: Vec<Task>,
    pub username: String,
}
