use serde::{Deserialize, Serialize};

use super::calendar::Calendar;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CalendarDb {
    pub calendar_date: String,
    pub calendar: [Calendar; 24],
    pub username: String,
}
