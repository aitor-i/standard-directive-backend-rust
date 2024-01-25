use crate::domain::models::response::Response;
use crate::{
    application::validate_token::validate_token,
    data_access::tasks::get_tasks_from_db::get_tasks_from_db,
};
use serde::{Deserialize, Serialize};
use warp::{reject::Rejection, reply::Reply, Filter};

#[derive(Debug, Serialize, Deserialize)]
struct QueryParams {
    token: String,
}

pub fn get_tasks_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("get-tasks"))
        .and(warp::query::<QueryParams>())
        .and_then(request_handler)
}

async fn request_handler(params: QueryParams) -> Result<Box<impl Reply>, Rejection> {
    let token = match validate_token(&params.token) {
        Ok(token) => token,
        Err(err) => {
            let message = err.to_string();
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )));
        }
    };

    match get_tasks_from_db(token.username).await {
        Ok(tasks_from_db) => {
            let message = Response::task_response("Ok".to_string(), tasks_from_db.unwrap().tasks);
            Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )))
        }
        Err(err) => {
            let message = Response::message_only(err.to_string());
            Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )))
        }
    }
}
