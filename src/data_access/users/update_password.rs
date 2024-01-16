use bson::doc;
use mongodb::error::Error;
use mongodb::Client;

use crate::domain::models::user::User;

pub async fn update_password(username: String, new_password: String) -> mongodb::error::Result<()> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "users";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<User>(collection_name);

    let filter = doc! {"username": username};
    let update = doc! { "$set": { "password": new_password} };

    match collection.update_one(filter, update, None).await {
        Ok(colletion) => {
            if colletion.matched_count == 0 {
                let custom_error = Error::custom("User not found");
                return Err(custom_error);
            };
            return Ok(());
        }

        Err(err) => {
            println!("Error or saving { }", err);
            return Err(err);
        }
    }
}
