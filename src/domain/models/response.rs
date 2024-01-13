use serde::{Deserialize, Serialize};

use super::calendar_to_save::CalendarToSave;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    message: String,
    token: Option<String>,
    username: Option<String>,
    calendar: Option<CalendarToSave>,
}

impl Response {
    pub fn message_only(message: String) -> Response {
        Response {
            message,
            token: None,
            username: None,
            calendar: None,
        }
    }

    pub fn login_response(message: String, token: String, username: String) -> Response {
        Response {
            message,
            token: Some(token),
            username: Some(username),
            calendar: None,
        }
    }
}
