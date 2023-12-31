use warp::{Filter, Rejection, Reply};

use crate::data_access::get_calendars_from_db::get_calendars_from_db;

async fn request_mapper() -> Result<impl Reply, Rejection> {
    let db_res = get_calendars_from_db().await;
    match db_res {
        Ok(calendars) => Ok(warp::reply::json(&calendars)),
        Err(err) => {
            let error_message = err.to_string();
            Ok(warp::reply::json(&error_message))
        }
    }
}

pub fn get_calendars_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("get-calendars")
        .and(warp::get())
        .and_then(request_mapper)
}
