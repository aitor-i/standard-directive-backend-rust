use serde::{Deserialize, Serialize};
use warp::Filter;

pub mod application;
pub mod controllers;
pub mod data_access;
pub mod domain;

use controllers::post_save_event_controller::post_save_event_controller;

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

    let hello_controller = warp::path("hello").and(warp::get()).map(|| {
        let response = Response {
            message: "hello rust".into(),
        };
        warp::reply::json(&response)
    });

    let hello_post_controller = warp::path("post-hello")
        .and(warp::post())
        .and(warp::body::json::<PostRequest>())
        .map(|body: PostRequest| warp::reply::json(&body.message));

    let root_path_controller = warp::get().map(|| {
        let response = "Service is up!";
        warp::reply::json(&response)
    });

    let router = hello_controller
        .or(root_path_controller)
        .or(hello_post_controller)
        .or(post_save_event_controller());

    warp::serve(router).run(([127, 0, 0, 1], 4040)).await;
}
