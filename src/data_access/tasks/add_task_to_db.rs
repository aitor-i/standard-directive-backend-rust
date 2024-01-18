use crate::{application::struct_to_document::struct_to_document, domain::models::task::Task};

use bson::Document;
use mongodb::{Client, Collection};

pub async fn add_task_to_db(task: Task) -> mongodb::error::Result<()> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "tasks";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let task_document = struct_to_document(&task).expect("Error");
    collection.insert_one(task_document, None).await?;

    Ok(())
}
