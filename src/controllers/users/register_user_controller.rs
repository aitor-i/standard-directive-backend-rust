use warp::{Filter, Rejection, Reply};

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

async fn request_mapper(mut body: User) -> Result<impl Reply, Rejection> {
    if let Ok(false) = is_username_free(&body.username).await {
        return Ok(warp::reply::json(&"userna is taken"));
    }
    let hashed_password = hash_password(body.password.clone());
    match hashed_password {
        Ok(hased) => body.password = hased,
        Err(err) => return Ok(warp::reply::json(&err.to_string())),
    };
    let db_res = add_user_db(&body).await;
    match db_res {
        Ok(()) => Ok(warp::reply::json(&body)),
        Err(err) => Ok(warp::reply::json(&err.to_string())),
    }
}
