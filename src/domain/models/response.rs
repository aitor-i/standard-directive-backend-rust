use serde::{Deserialize, Serialize};

use super::{calendar_to_save::CalendarToSave, task::Task};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    message: String,
    token: Option<String>,
    username: Option<String>,
    calendar: Option<CalendarToSave>,
    tasks: Option<Vec<Task>>,
}

impl Response {
    pub fn message_only(message: String) -> Response {
        Response {
            message,
            token: None,
            username: None,
            calendar: None,
            tasks: None,
        }
    }

    pub fn login_response(message: String, token: String, username: String) -> Response {
        Response {
            message,
            token: Some(token),
            username: Some(username),
            calendar: None,
            tasks: None,
        }
    }

    pub fn task_response(message: String, tasks: Vec<Task>) -> Response {
        Response {
            message,
            token: None,
            username: None,
            calendar: None,
            tasks: Some(tasks),
        }
    }

    pub fn calendar_response(message: String, calendar: CalendarToSave) -> Response {
        Response {
            message,
            token: None,
            username: None,
            calendar: Some(calendar),
            tasks: None,
        }
    }
}
