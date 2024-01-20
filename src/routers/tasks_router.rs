use warp::{reject::Rejection, reply::Reply, Filter};

use crate::controllers::tasks::{
    get_tasks_controller::get_tasks_controller, post_add_task_controller::post_add_task_controller,
};

pub fn tasks_router() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("tasks").and(post_add_task_controller().or(get_tasks_controller()))
}
