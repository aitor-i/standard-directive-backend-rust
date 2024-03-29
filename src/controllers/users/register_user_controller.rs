use warp::{Filter, Rejection, Reply};

use crate::application::generate_token::generate_token;
use crate::data_access::users::is_code_valid::is_code_valid;
use crate::data_access::users::is_email_free::is_email_free;
use crate::domain::models::auth_token::AuthToken;
use crate::domain::models::response::Response;
use crate::{
    application::has_password::hash_password,
    data_access::users::{add_user_db::add_user_db, is_username_free::is_username_free},
    domain::models::user::User,
};

pub fn register_user_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("register-user")
        .and(warp::post())
        .and(warp::body::json::<User>())
        .and_then(request_mapper)
        .boxed()
}

async fn request_mapper(mut body: User) -> Result<Box<dyn Reply>, Rejection> {
    if let Ok(false) = is_code_valid(&body.code).await {
        let message = Response::message_only("Invalid code".to_string());
        return Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&message),
            warp::http::StatusCode::BAD_REQUEST,
        )));
    }
    if let Ok(false) = is_username_free(&body.username).await {
        let message = Response::message_only("Username is taken".to_string());
        return Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&message),
            warp::http::StatusCode::BAD_REQUEST,
        )));
    }

    if let Ok(false) = is_email_free(&body.email).await {
        let message = Response::message_only("Email is taken".to_string());
        return Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&message),
            warp::http::StatusCode::BAD_REQUEST,
        )));
    }

    let hashed_password = hash_password(body.password.clone());
    match hashed_password {
        Ok(hased) => body.password = hased,
        Err(err) => {
            let message = Response::message_only(err.to_string());
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )));
        }
    };

    let db_res = add_user_db(&body).await;
    match db_res {
        Ok(()) => {
            let auth_token = AuthToken::without_roles(body.username.clone());
            let token = match generate_token(auth_token) {
                Ok(token_ok) => token_ok,
                Err(_) => "Error on token generation".to_string(),
            };
            let message = Response::login_response(
                "Success on register-user".to_string(),
                token,
                body.username,
            );
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::OK,
            )));
        }
        Err(err) => {
            println!("{}", err);
            let message = Response::message_only(err.to_string());
            return Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&message),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )));
        }
    }
}
