use serde::{Deserialize, Serialize};

use crate::domain::models::event::Event;

#[derive(Deserialize, Serialize)]
pub struct CalendarToSave {
    pub calendar_date: String,
    pub calendar_id: CalendarId,
    pub events: [Event; 24],
}

#[derive(Deserialize, Serialize)]
pub struct CalendarId(String);

#[derive(Deserialize, Serialize)]
struct UserId(String);
