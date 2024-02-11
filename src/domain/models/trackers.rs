use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct TrackerObject {
    title: String,
    pub id: String,
    pub days: Vec<DayObject>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DayObject {
    completed: bool,
    day: u8,
    color: String,
    year: u16,
    month: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackerDate {
    year: u8,
    month: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackersDB {
    pub trackers: Vec<TrackerObject>,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackerViewModel {
    pub trackers: Vec<TrackerObject>,
    pub token: String,
}
