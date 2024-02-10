use crate::{
    application::struct_to_document::struct_to_document, domain::models::trackers::TrackersDB,
};

use bson::Document;
use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

pub async fn add_tracker_to_db(trackers: TrackersDB) -> mongodb::error::Result<()> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "trackers";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let tracker_document = struct_to_document(&trackers).expect("Error");
    collection.insert_one(tracker_document, None).await?;

    Ok(())
}
