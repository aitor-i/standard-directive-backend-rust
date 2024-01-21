use bson::{doc, Document};
use mongodb::error::Error;
use mongodb::{Client, Collection};

use crate::domain::models::task::Task;

pub async fn update_tasks(username: String, tasks: Vec<Task>) -> mongodb::error::Result<()> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "tasks";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    // Define the filter for the document you want to update
    let filter = doc! { "username": &username };

    match bson::to_bson(&tasks) {
        Ok(tasks_bson) => {
            let update = doc! { "$set": { "tasks": tasks_bson} };

            match collection.update_one(filter, update, None).await {
                Ok(colletion) => {
                    println!("Update");
                    if colletion.matched_count == 0 {
                        let custom_error = Error::custom("No tasks found");
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
