use crate::{
    application::{generate_token::generate_token, is_password_correct::is_password_correct},
    data_access::users::find_user_by_username::find_user_by_username,
    domain::models::{auth_token::AuthToken, user::User},
};
use warp::{Filter, Rejection, Reply};

use crate::domain::models::response::Response;

pub fn login_controller() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json::<User>())
        .and_then(request_mapper)
        .boxed()
}
async fn request_mapper(body: User) -> Result<Box<dyn Reply>, Rejection> {
    let db_res = find_user_by_username(&body.username).await;
    if let Ok(Some(user)) = &db_res {
        if is_password_correct(body.password.clone(), &user.password.clone()) {
            let auth_token = AuthToken::without_roles(body.username.clone());
            let token = match generate_token(auth_token) {
                Ok(token_ok) => token_ok,
                Err(_) => "Error on token generation".to_string(),
            };
            let response =
                Response::login_response("Loged!".to_string(), token, body.username.clone());
            Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&response),
                warp::http::StatusCode::OK,
            )))
        } else {
            return {
                let response = Response::message_only("Invalid username or password".to_string());
                Ok(Box::new(warp::reply::with_status(
                    warp::reply::json(&response),
                    warp::http::StatusCode::UNAUTHORIZED,
                )))
            };
        }
    } else {
        return {
            let response = Response::message_only("Invalid username or password".to_string());
            Ok(Box::new(warp::reply::with_status(
                warp::reply::json(&response),
                warp::http::StatusCode::UNAUTHORIZED,
            )))
        };
    }
}
