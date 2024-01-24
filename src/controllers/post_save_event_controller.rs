use warp::{Filter, Rejection, Reply};

use crate::application::validate_token::validate_token;
use crate::data_access::add_event_to_db::add_event_to_db;
use crate::data_access::update_calendar::update_calendar;
use crate::domain::models::calendar_db::CalendarDb;
use crate::domain::models::calendar_to_save::CalendarToSave;
use crate::domain::models::response::Response;

async fn request_mapper(body: CalendarToSave) -> Result<Box<dyn Reply>, Rejection> {
    match validate_token(&body.token) {
        Ok(token) => {
            let calendar = body.calendar.clone();
            let calendar_date = body.calendar_date.clone();
            let username = token.username;

            let calendar_for_db = CalendarDb {
                calendar_date,
                calendar,
                username,
            };

            match update_calendar(&calendar_for_db).await {
                Ok(_) => return Ok(Box::new(warp::reply::json(&body))),
                Err(_) => match add_event_to_db(&calendar_for_db).await {
                    Ok(_) => return Ok(Box::new(warp::reply::json(&body))),
                    Err(err) => {
                        let message = Response::message_only(err.to_string());
                        return Ok(Box::new(warp::reply::with_status(
                            warp::reply::json(&message),
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        )));
                    }
                },
            };
        }
        Err(err) => Ok(Box::new(warp::reply::with_status(
            err.to_string(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))),
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
