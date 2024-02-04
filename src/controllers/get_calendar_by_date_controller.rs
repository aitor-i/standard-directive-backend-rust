use crate::domain::models::response::Response;
use serde::Deserialize;
use warp::{Filter, Rejection, Reply};

use crate::{
    application::validate_token::validate_token,
    data_access::get_calendar_by_date::get_calendar_by_date,
};

#[derive(Deserialize)]
struct QueryParams {
    date: String,
    token: String,
}

async fn request_mapper(params: QueryParams) -> Result<Box<dyn Reply>, Rejection> {
    let date = params.date.clone();

    match validate_token(&params.token) {
        Ok(token) => {
            let db_res = get_calendar_by_date(date, token.username).await;
            match db_res {
                Ok(Some(calendar)) => {
                    let message =
                        Response::calendar_response("Seuccess".to_string(), calendar.calendar);
                    Ok(Box::new(warp::reply::json(&message)))
                }
                Ok(None) => {
                    let message = Response::message_only("No callendar founs".to_string());
                    Ok(Box::new(warp::reply::json(&message)))
                }
                Err(err) => {
                    println!("{}", err);
                    let message = "Error getting calendars";
                    Ok(Box::new(warp::reply::with_status(
                        warp::reply::json(&message),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )))
                }
            }
        }
        Err(err) => {
            let message = Response::message_only(err.to_string());
            Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::UNAUTHORIZED,
            )))
        }
    }
}

pub fn get_calendar_by_date_controller(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("get-calendar-by-date"))
        .and(warp::query::<QueryParams>())
        .and_then(request_mapper)
}
