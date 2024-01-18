use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    id: String,
    taskName: String,
    isCompleted: bool,
    isSelected: bool,
}
