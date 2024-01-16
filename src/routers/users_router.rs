use warp::Filter;

use crate::controllers::users::login_controller::login_controller;
use crate::controllers::users::register_user_controller::register_user_controller;
use crate::controllers::users::update_password_controller::update_password_controller;

pub fn users_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("users").and(
        register_user_controller()
            .or(login_controller())
            .or(update_password_controller()),
    )
}
