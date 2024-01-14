use serde::{Deserialize, Serialize};

use super::calendar::Calendar;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CalendarToSave {
    pub calendar_date: String,
    pub calendar: [Calendar; 24],
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CalendarId(String);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserId(String);
