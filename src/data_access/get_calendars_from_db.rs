use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Client;
use serde::{Deserialize, Serialize};

use crate::domain::models::calendar_to_save::CalendarToSave;

#[derive(Serialize, Deserialize)]
struct CalendarModel {
    calendar_id: String,
}

pub async fn get_calendars_from_db() -> Result<Vec<CalendarToSave>, mongodb::error::Error> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "calendars";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<CalendarToSave>(collection_name);

    let filter = doc! {};
    let mut cursor = collection.find(filter, None).await?;

    let mut calendars: Vec<CalendarToSave> = vec![];
    while let Some(calendar) = cursor.try_next().await? {
        calendars.push(calendar);
    }

    Ok(calendars)
}
