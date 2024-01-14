use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    color: Option<String>,
    eventName: String,
    eventPosition: Option<String>,
    hour: u8,
    hourDisplay: String,
    taskId: Option<usize>,
    isCompleted: bool,
}
