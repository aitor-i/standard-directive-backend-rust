use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CalendarToSave {
    pub calendar_date: String,
    pub calendar_id: CalendarId,
    // pub events: [Event; 24],
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CalendarId(String);

#[derive(Deserialize, Serialize, Debug)]
struct UserId(String);
