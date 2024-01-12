use crate::{
    application::is_password_correct::is_password_correct,
    data_access::users::find_user_by_username::find_user_by_username, domain::models::user::User,
};
use warp::{Filter, Rejection, Reply};

pub fn login_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json::<User>())
        .and_then(request_mapper)
        .boxed()
}

async fn request_mapper(body: User) -> Result<impl Reply, Rejection> {
    let db_res = find_user_by_username(&body.username).await;
    if let Ok(Some(user)) = &db_res {
        if is_password_correct(body.password.clone(), &user.password.clone()) {
            return Ok(warp::reply::json(&"loged!!"));
        } else {
            return Ok(warp::reply::json(&"invalid username or password"));
        }
    } else {
        return Ok(warp::reply::json(&"invalid username or password"));
    }
}
