use mongodb::bson::doc;

use mongodb::Client;

use dotenv::dotenv;
use std::env;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Code {
    code: String,
}

pub async fn is_code_valid(code: &String) -> Result<bool, mongodb::error::Error> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "code";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<Code>(collection_name);

    let filter = doc! { "code": code};
    let code = collection.find_one(filter, None).await?;

    match code {
        Some(_) => return Ok(false),
        None => return Ok(false),
    };
}
