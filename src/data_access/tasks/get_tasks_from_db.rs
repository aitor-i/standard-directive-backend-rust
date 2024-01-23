use bson::doc;
use mongodb::Client;

use crate::domain::models::save_tasks_view_model::TaskDbModel;

use dotenv::dotenv;
use std::env;
pub async fn get_tasks_from_db(
    username: String,
) -> Result<Option<TaskDbModel>, mongodb::error::Error> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "tasks";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<TaskDbModel>(collection_name);

    let filter = doc! {"username": username};
    let tasks_model = collection.find_one(filter, None).await?;

    Ok(tasks_model)
}
