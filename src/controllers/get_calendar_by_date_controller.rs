use serde::Deserialize;
use warp::{Filter, Rejection, Reply};

use crate::{
    application::{convert_string_to_date::convert_string_to_date, validate_token::validate_token},
    data_access::get_calendar_by_date::get_calendar_by_date,
};

#[derive(Deserialize)]
struct QueryParams {
    date: String,
    token: String,
}

async fn request_mapper(params: QueryParams) -> Result<impl Reply, Rejection> {
    let date = params.date.clone();

    match validate_token(&params.token) {
        Ok(token) => {
            println!("username: {}, date: {}", &token.username, &date);
            let db_res = get_calendar_by_date(date, token.username).await;
            match db_res {
                Ok(calendar) => Ok(warp::reply::json(&calendar)),
                Err(err) => {
                    let error_message = err.to_string();
                    Ok(warp::reply::json(&error_message))
                }
            }
        }
        Err(err) => Ok(warp::reply::json(&err.to_string())),
    }
}

pub fn get_calendar_by_date_controller(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("get-calendar-by-date"))
        .and(warp::query::<QueryParams>())
        .and_then(request_mapper)
}
