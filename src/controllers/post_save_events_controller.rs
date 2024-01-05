use chrono::{NaiveDateTime, ParseError};
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

use standard_directive_n2_backend::domain::models::event::Event;

#[derive(Deserialize, Serialize)]
struct EventsToSave {
    event_date: String,
    event_id: EventId,
    events: [Event; 24],
}

#[derive(Deserialize, Serialize)]
struct EventId(String);

#[derive(Deserialize, Serialize)]
struct UserId(String);

#[derive(Deserialize, Serialize)]
struct Message {
    mesage: String,
}

fn convert_string_to_date(date_in_string_format: &String) -> Result<NaiveDateTime, ParseError> {
    let format = "%Y-%m-%d %H:%M:%S";
    let naive_date = NaiveDateTime::parse_from_str(&date_in_string_format, format);

    return naive_date;
}

pub fn post_save_event_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    warp::path("save-events")
        .and(warp::post())
        .and(warp::body::json::<EventsToSave>())
        .map(|body: EventsToSave| {
            let formated_date = convert_string_to_date(&body.event_date);
            match formated_date {
                Ok(formated_date) => {
                    println!("{:?}", &formated_date);
                    warp::reply::json(&body)
                }
                Err(err) => {
                    let error_message = err.to_string();
                    warp::reply::json(&error_message)
                }
            }
        })
}
