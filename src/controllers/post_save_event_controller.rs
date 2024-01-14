use warp::{Filter, Rejection, Reply};

use crate::application::validate_token::validate_token;
use crate::data_access::add_event_to_db::add_event_to_db;
use crate::domain::models::calendar_db::CalendarDb;
use crate::domain::models::calendar_to_save::CalendarToSave;

async fn request_mapper(body: CalendarToSave) -> Result<impl Reply, Rejection> {
    match validate_token(&body.token) {
        Ok(token) => {
            let calendar = body.calendar.clone();
            let calendar_date = body.calendar_date.clone();
            let username = token.username;

            println!("Time of token {}", token.exp);

            let calendar_for_db = CalendarDb {
                calendar_date,
                calendar,
                username,
            };

            match add_event_to_db(&calendar_for_db).await {
                Ok(_) => Ok(warp::reply::json(&body)),
                Err(err) => Ok(warp::reply::json(&err.to_string())),
            }
        }
        Err(err) => Ok(warp::reply::json(&err.to_string())),
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
