use serde::{Deserialize, Serialize};

pub mod application;
pub mod controllers;
pub mod data_access;
pub mod domain;
pub mod routers;

use routers::calendar_router::calendar_router;
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

    let router = calendar_router().or(users_router());

    warp::serve(router).run(([127, 0, 0, 1], 4040)).await;
}
