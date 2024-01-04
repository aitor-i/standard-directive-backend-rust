use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

#[derive(Deserialize, Serialize)]
struct EventsToSave {
    event_date: String,
}

#[derive(Deserialize, Serialize)]
struct EventId(String);
struct Event {
    task_id: String,
}

#[derive(Deserialize, Serialize)]
struct UserId(String);

pub fn post_save_event_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    warp::path("save-events")
        .and(warp::post())
        .and(warp::body::json::<EventsToSave>())
        .map(|body: EventsToSave| warp::reply::json(&body))
}
