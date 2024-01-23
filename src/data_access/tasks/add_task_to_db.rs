use crate::{
    application::struct_to_document::struct_to_document,
    domain::models::save_tasks_view_model::TaskDbModel,
};

use bson::Document;
use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

pub async fn add_task_to_db(task: TaskDbModel) -> mongodb::error::Result<()> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "tasks";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let task_document = struct_to_document(&task).expect("Error");
    collection.insert_one(task_document, None).await?;

    Ok(())
}
