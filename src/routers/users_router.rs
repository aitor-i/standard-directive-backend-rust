use warp::Filter;

use crate::controllers::post_save_event_controller::post_save_event_controller;

use crate::controllers::users::register_user_controller::register_user_controller;

pub fn users_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("users").and(register_user_controller().or(post_save_event_controller()))
}
