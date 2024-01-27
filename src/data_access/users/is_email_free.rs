use crate::domain::models::user::User;
use mongodb::bson::doc;

use mongodb::Client;

use dotenv::dotenv;
use std::env;

pub async fn is_email_free(email: &String) -> Result<bool, mongodb::error::Error> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "users";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<User>(collection_name);

    let filter = doc! { "email": email};
    let user = collection.find_one(filter, None).await?;

    match user {
        Some(_) => return Ok(false),
        None => return Ok(true),
    };
}
