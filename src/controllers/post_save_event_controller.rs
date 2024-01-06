use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

use crate::application::convert_string_to_date::convert_string_to_date;
use crate::data_access::add_event_to_db::add_event_to_db;
use crate::domain::models::calendar_to_save::CalendarToSave;
#[derive(Deserialize, Serialize)]
struct Message {
    mesage: String,
}

async fn request_mapper(body: CalendarToSave) -> Result<impl Reply, Rejection> {
    let formated_date = convert_string_to_date(&body.calendar_date);
    match formated_date {
        Ok(formated_date) => {
            println!("{:?}", &formated_date);
            let _ = add_event_to_db(&body).await;
            Ok(warp::reply::json(&body))
        }
        Err(err) => {
            let error_message = err.to_string();
            Ok(warp::reply::json(&error_message))
        }
    }
}

pub fn post_save_event_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    warp::path("save-events")
        .and(warp::post())
        .and(warp::body::json::<CalendarToSave>())
        .and_then(request_mapper)
        .boxed()
}
