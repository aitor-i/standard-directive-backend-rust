use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CalendarToSave {
    pub calendar_date: String,
    pub calendar_id: CalendarId,
    pub user_id: UserId,
    // pub events: [Event; 24],
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CalendarId(String);

#[derive(Deserialize, Serialize, Debug)]
pub struct UserId(String);
