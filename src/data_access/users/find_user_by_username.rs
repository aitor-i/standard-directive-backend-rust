use crate::domain::models::user::User;
use mongodb::bson::doc;

use mongodb::Client;

use dotenv::dotenv;
use mongodb::options::ClientOptions;
use std::env;
pub async fn find_user_by_username(
    username: &String,
) -> Result<Option<User>, mongodb::error::Error> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "users";

    let mut client_options = ClientOptions::parse_async(connection_string).await?;
    client_options.max_pool_size = Some(20);

    let client = Client::with_options(client_options)?;

    let database = client.database(db_name);
    let collection = database.collection::<User>(collection_name);

    let filter = doc! { "username": username};
    let user = collection.find_one(filter, None).await?;

    Ok(user)
}
