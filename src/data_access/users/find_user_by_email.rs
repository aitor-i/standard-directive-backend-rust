use crate::domain::models::user::User;
use mongodb::bson::doc;

use mongodb::Client;

use dotenv::dotenv;
use std::env;
pub async fn find_user_by_email(email: &String) -> Result<Option<User>, mongodb::error::Error> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "users";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<User>(collection_name);

    let filter = doc! { "email": email};
    let user = collection.find_one(filter, None).await?;

    Ok(user)
}
