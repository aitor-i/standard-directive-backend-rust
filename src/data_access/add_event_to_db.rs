use bson::{doc, Document};
use mongodb::{Client, Collection};

pub async fn add_event_to_db() -> mongodb::error::Result<()> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "calendars";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let new_doc = doc! {
        "title": "Rust and MongoDB",
        "content": "Using MongoDB in a Rust project",
    };

    collection.insert_one(new_doc, None).await?;

    Ok(())
}
