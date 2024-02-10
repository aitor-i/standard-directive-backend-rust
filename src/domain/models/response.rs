use serde::{Deserialize, Serialize};

use super::{calendar::Calendar, task::Task, trackers::TrackerObject};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    message: String,
    token: Option<String>,
    username: Option<String>,
    calendar: Option<[Calendar; 24]>,
    tasks: Option<Vec<Task>>,
    trackers: Option<Vec<TrackerObject>>,
}

impl Response {
    pub fn message_only(message: String) -> Response {
        Response {
            message,
            token: None,
            username: None,
            calendar: None,
            tasks: None,
            trackers: None,
        }
    }

    pub fn login_response(message: String, token: String, username: String) -> Response {
        Response {
            message,
            token: Some(token),
            username: Some(username),
            calendar: None,
            tasks: None,
            trackers: None,
        }
    }

    pub fn task_response(message: String, tasks: Vec<Task>) -> Response {
        Response {
            message,
            token: None,
            username: None,
            calendar: None,
            tasks: Some(tasks),
            trackers: None,
        }
    }

    pub fn tracker_response(message: String, trackers: Vec<TrackerObject>) -> Response {
        Response {
            message,
            token: None,
            username: None,
            calendar: None,
            tasks: None,
            trackers: Some(trackers),
        }
    }

    pub fn calendar_response(message: String, calendar: [Calendar; 24]) -> Response {
        Response {
            message,
            token: None,
            username: None,
            calendar: Some(calendar),
            tasks: None,
            trackers: None,
        }
    }
}
