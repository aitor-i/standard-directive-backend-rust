use warp::{Filter, Rejection, Reply};

use crate::domain::models::response::Response;
use crate::{
    application::has_password::hash_password,
    data_access::users::{add_user_db::add_user_db, is_username_free::is_username_free},
    domain::models::user::User,
};

pub fn register_user_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("register-user")
        .and(warp::post())
        .and(warp::body::json::<User>())
        .and_then(request_mapper)
        .boxed()
}

async fn request_mapper(mut body: User) -> Result<Box<dyn Reply>, Rejection> {
    if let Ok(false) = is_username_free(&body.username).await {
        let message = Response::message_only("Username is taken".to_string());
        return Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&message),
            warp::http::StatusCode::BAD_REQUEST,
        )));
    }
    let hashed_password = hash_password(body.password.clone());
    match hashed_password {
        Ok(hased) => body.password = hased,
        Err(err) => {
            let message = Response::message_only(err.to_string());
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::UNAUTHORIZED,
            )));
        }
    };
    let db_res = add_user_db(&body).await;
    match db_res {
        Ok(()) => {
            let message = Response::message_only("Success".to_string());
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::OK,
            )));
        }
        Err(err) => {
            let message = Response::message_only(err.to_string());
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::UNAUTHORIZED,
            )));
        }
    }
}
