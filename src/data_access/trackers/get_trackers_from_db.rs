use bson::doc;
use mongodb::Client;

use dotenv::dotenv;
use std::env;

use crate::domain::models::trackers::TrackersDB;
pub async fn get_trackers_from_db(
    username: String,
) -> Result<Option<TrackersDB>, mongodb::error::Error> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "trackers";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<TrackersDB>(collection_name);

    let filter = doc! {"username": username};
    let trackers_model = collection.find_one(filter, None).await?;

    Ok(trackers_model)
}
