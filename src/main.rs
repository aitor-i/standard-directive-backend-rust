use serde::{Deserialize, Serialize};

pub mod application;
pub mod controllers;
pub mod data_access;
pub mod domain;
pub mod routers;

use routers::calendar_router::calendar_router;
use routers::tasks_router::tasks_router;
use routers::users_router::users_router;
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct PostRequest {
    message: String,
}

#[tokio::main]
async fn main() {
    #[derive(Deserialize, Serialize)]
    struct Response {
        message: String,
    }
    let cors = warp::cors()
        .allow_any_origin() // or specify like .allow_origin("http://example.com")
        .allow_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"])
        .allow_headers(vec!["content-type", "Authorization"])
        .build();

    let root_route = warp::path::end().and_then(root_handler);

    let router = root_route
        .or(calendar_router())
        .or(users_router())
        .or(tasks_router())
        .with(cors);

    println!("server runing on port 4040");
    warp::serve(router).run(([0, 0, 0, 0], 4040)).await;
}

pub async fn root_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "Server is up",
        warp::http::StatusCode::OK,
    ))
}
