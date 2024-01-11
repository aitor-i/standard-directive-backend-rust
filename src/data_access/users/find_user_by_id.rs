use crate::domain::models::user::User;
use mongodb::bson::doc;

use mongodb::Client;

pub async fn find_user_by_id(id: &String) -> Result<Option<User>, mongodb::error::Error> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "users";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<User>(collection_name);

    let filter = doc! { "user_id": id};
    let user = collection.find_one(filter, None).await?;

    Ok(user)
}
