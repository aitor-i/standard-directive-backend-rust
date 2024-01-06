use crate::domain::models::calendar_to_save::CalendarToSave;
use bson::{doc, ser::Error, to_bson, Bson, Document};
use mongodb::{Client, Collection};
use serde::ser::Error as SerdeError;

pub async fn add_event_to_db(calendar: &CalendarToSave) -> mongodb::error::Result<()> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "calendars";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let calendar_document = struct_to_document(&calendar).expect("Error");
    let new_doc = doc! {
        "title": "Rust and MongoDB",
        "content": "Using MongoDB in a Rust project",
    };

    collection.insert_one(calendar_document, None).await?;

    Ok(())
}

fn struct_to_document(calendar: &CalendarToSave) -> Result<Document, Error> {
    match to_bson(calendar) {
        Ok(Bson::Document(document)) => Ok(document),
        Ok(_) => Err(Error::custom("Expected a BSON document.")),
        Err(e) => Err(e), // Propagate other errors
    }
}
