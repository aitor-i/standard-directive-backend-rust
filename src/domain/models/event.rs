use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Event {
    hour_display: String,
    hour: u8,
    color: Option<String>,
    event_name: String,
    event_position: Option<String>,
    task_id: u8,
    is_completed: Option<bool>,
}
