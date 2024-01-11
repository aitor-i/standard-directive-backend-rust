use warp::{Filter, Rejection, Reply};

use crate::controllers::users::register_user_controller::register_user_controller;

pub fn users_router() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("users").and(register_user_controller())
}
