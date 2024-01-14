use bson::{doc, Document};
use mongodb::error::Error;
use mongodb::{Client, Collection};

use crate::domain::models::calendar_db::CalendarDb;

pub async fn update_calendar(calendar: &CalendarDb) -> mongodb::error::Result<()> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "calendars";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    // Define the filter for the document you want to update
    let filter = doc! { "username": &calendar.username, "calendar_date": &calendar.calendar_date };

    match bson::to_bson(&calendar.calendar) {
        Ok(calendar_bson) => {
            let update = doc! { "$set": { "calendar": calendar_bson} };

            match collection.update_one(filter, update, None).await {
                Ok(colletion) => {
                    println!("Update");
                    if colletion.matched_count == 0 {
                        let custom_error = Error::custom("No calendar found");
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
