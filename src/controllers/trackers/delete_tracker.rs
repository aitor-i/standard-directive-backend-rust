use std::string;

use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

use crate::data_access::trackers::add_trackers_db::add_tracker_to_db;
use crate::data_access::trackers::get_trackers_from_db::get_trackers_from_db;
use crate::domain::models::response::Response;

use crate::application::validate_token::validate_token;
use crate::domain::models::trackers::{TrackerObject, TrackerViewModel, TrackersDB};

#[derive(Debug, Serialize, Deserialize)]
struct QueryParams {
    token: String,
    id: String,
}

pub fn delete_tracker() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::delete()
        .and(warp::path("delete-tracker"))
        .and(warp::query::<QueryParams>())
        .and_then(repuest_handler)
}
async fn repuest_handler(params: QueryParams) -> Result<impl Reply, Rejection> {
    let token = match validate_token(&params.token) {
        Ok(token) => token,
        Err(_) => {
            let message = "Error on token validation".to_string();
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::UNAUTHORIZED,
            )));
        }
    };

    let trackers = match get_trackers_from_db(token.username.clone()).await {
        Ok(Some(trackers_from_db)) => trackers_from_db.trackers,
        Ok(None) => vec![] as Vec<TrackerObject>,
        Err(_) => vec![] as Vec<TrackerObject>,
    };

    let new_tracker = trackers
        .into_iter()
        .filter(|tracker| tracker.id != params.id)
        .collect();

    let tracks_for_db = TrackersDB {
        username: token.username.clone(),
        trackers: new_tracker,
    };

    match add_tracker_to_db(tracks_for_db).await {
        Ok(_) => {
            let message = Response::message_only("Tracker deleted".to_string());
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
