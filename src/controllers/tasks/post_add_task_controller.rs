use warp::{Filter, Rejection, Reply};

use crate::domain::models::response::Response;

use crate::domain::models::save_tasks_view_model::{SaveTasksViewModel, TaskDbModel};
use crate::{
    application::validate_token::validate_token, data_access::tasks::add_task_to_db::add_task_to_db,
};

pub fn post_add_task_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("save-tasks"))
        .and(warp::body::json::<SaveTasksViewModel>())
        .and_then(repuest_handler)
        .boxed()
}
async fn repuest_handler(body: SaveTasksViewModel) -> Result<impl Reply, Rejection> {
    let token = match validate_token(&body.token) {
        Ok(token) => token,
        Err(_) => return Ok(warp::reply::json(&"Error on token validation")),
    };

    let task_for_db = TaskDbModel {
        username: token.username,
        tasks: body.tasks,
    };

    match add_task_to_db(task_for_db).await {
        Ok(_) => {
            let message = Response::message_only("Task add successfully".to_string());
            return Ok(warp::reply::json(&message));
        }
        Err(err) => {
            let message = Response::message_only(err.to_string());
            return Ok(warp::reply::json(&message));
        }
    }
}
