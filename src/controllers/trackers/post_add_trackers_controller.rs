use warp::{Filter, Rejection, Reply};

use crate::data_access::trackers::add_trackers_db::add_tracker_to_db;
use crate::domain::models::response::Response;

use crate::application::validate_token::validate_token;
use crate::domain::models::trackers::{TrackerViewModel, TrackersDB};

pub fn post_add_tracker_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    warp::post()
        .and(warp::path("save-trackers"))
        .and(warp::body::json::<TrackerViewModel>())
        .and_then(repuest_handler)
        .boxed()
}
async fn repuest_handler(body: TrackerViewModel) -> Result<impl Reply, Rejection> {
    let token = match validate_token(&body.token) {
        Ok(token) => token,
        Err(_) => {
            let message = "Error on token validation".to_string();
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::UNAUTHORIZED,
            )));
        }
    };

    let tracks_for_db = TrackersDB {
        username: token.username,
        trackers: body.trackers,
    };

    match add_tracker_to_db(tracks_for_db).await {
        Ok(_) => {
            let message = Response::message_only("Trackers add successfully".to_string());
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::OK,
            )));
        }
        Err(err) => {
            let message = Response::message_only(err.to_string());
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )));
        }
    }
}
