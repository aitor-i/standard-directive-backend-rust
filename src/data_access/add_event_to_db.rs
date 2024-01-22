use crate::{
    application::struct_to_document::struct_to_document, domain::models::calendar_db::CalendarDb,
};
use bson::Document;
use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

pub async fn add_event_to_db(calendar: &CalendarDb) -> mongodb::error::Result<()> {
    dotenv().ok();

    let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "calendars";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let calendar_document = struct_to_document(&calendar).expect("Error");

    collection.insert_one(calendar_document, None).await?;

    Ok(())
}
