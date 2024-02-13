use warp::{reject::Rejection, reply::Reply, Filter};

use crate::controllers::trackers::{
    delete_tracker::delete_tracker, get_trackers_controller::get_trackers_controller,
    post_add_trackers_controller::post_add_tracker_controller,
};

pub fn trackers_router() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("trackers").and(
        post_add_tracker_controller()
            .or(get_trackers_controller())
            .or(delete_tracker()),
    )
}
