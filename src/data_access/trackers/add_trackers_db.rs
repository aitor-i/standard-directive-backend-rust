use crate::domain::models::trackers::TrackersDB;

use bson::{doc, Document};
use dotenv::dotenv;
use mongodb::error::Error;
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

    let filter = doc! { "username": trackers.username   };

    match bson::to_bson(&trackers.trackers) {
        Ok(trackers_bson) => {
            let update = doc! { "$set": { "trackers": trackers_bson} };

            match collection.update_one(filter, update, None).await {
                Ok(colletion) => {
                    println!("Update");
                    if colletion.matched_count == 0 {
                        let custom_error = Error::custom("No tracker found");
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
        Err(err) => {
            println!("Error on parsing {}", err);
            Ok(())
        }
    }
}
