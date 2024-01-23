use bson::Document;
use mongodb::{Client, Collection};

use crate::{application::struct_to_document::struct_to_document, domain::models::user::User};

use dotenv::dotenv;
use std::env;
pub async fn add_user_db(user: &User) -> mongodb::error::Result<()> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "users";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let user_document = struct_to_document(user).expect("Error");

    collection.insert_one(user_document, None).await?;

    Ok(())
}
