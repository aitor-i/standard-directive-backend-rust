use crate::domain::models::trackers::TrackersDB;

use bson::{doc, Document};
use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

pub async fn add_tracker_to_db(trackers: TrackersDB) -> mongodb::error::Result<()> {
    dotenv().ok();

    if trackers.trackers.len() == 0 {
        return Ok(());
    }

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "trackers";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);
    let insert_collection: Collection<TrackersDB> = database.collection(collection_name);

    let filter = doc! { "username": trackers.username.clone()   };

    match bson::to_bson(&trackers.trackers) {
        Ok(trackers_bson) => {
            let update = doc! { "$set": { "trackers": trackers_bson} };

            match collection.update_one(filter, update, None).await {
                Ok(colletion_res) => {
                    println!("Update");
                    if colletion_res.matched_count == 0 {
                        println!("No matched trackers");
                        // let custom_error = Error::custom("No tracker found");

                        let _ = insert_collection.insert_one(trackers, None).await;
                        return Ok(());
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
