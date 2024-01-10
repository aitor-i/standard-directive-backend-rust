use warp::{Filter, Rejection, Reply};

use crate::{data_access::users::add_user_db::add_user_db, domain::models::user::User};

pub fn register_user_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("register-user")
        .and(warp::post())
        .and(warp::body::json::<User>())
        .and_then(request_mapper)
        .boxed()
}

async fn request_mapper(body: User) -> Result<impl Reply, Rejection> {
    let db_res = add_user_db(&body).await;
    match db_res {
        Ok(()) => Ok(warp::reply::json(&body)),
        Err(err) => Ok(warp::reply::json(&err.to_string())),
    }
}
