use crate::application::struct_to_document::struct_to_document;
use crate::domain::models::calendar_to_save::CalendarToSave;
use bson::Document;
use mongodb::{Client, Collection};

pub async fn add_event_to_db(calendar: &CalendarToSave) -> mongodb::error::Result<()> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "calendars";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let calendar_document = struct_to_document(&calendar).expect("Error");

    collection.insert_one(calendar_document, None).await?;

    Ok(())
}
