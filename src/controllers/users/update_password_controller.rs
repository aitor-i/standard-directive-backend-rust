use warp::{Filter, Rejection, Reply};

use crate::{
    application::has_password::hash_password,
    data_access::users::update_password::update_password,
    domain::models::{response::Response, user::User},
};

pub fn update_password_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    warp::post()
        .and(warp::path("update-password"))
        .and(warp::path::end())
        .and(warp::body::json::<User>())
        .and_then(request_handler)
}

async fn request_handler(mut body: User) -> Result<impl Reply, Rejection> {
    let hashed_password = hash_password(body.password.clone());
    match hashed_password {
        Ok(hased) => body.password = hased,
        Err(err) => return Ok(warp::reply::json(&err.to_string())),
    };
    let db_res = update_password(body.username.clone(), body.password).await;
    match db_res {
        Ok(()) => {
            let message = Response::message_only("Password updated!".to_string());
            Ok(warp::reply::json(&message))
        }
        Err(err) => Ok(warp::reply::json(&err.to_string())),
    }
}
