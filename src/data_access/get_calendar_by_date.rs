use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::Client;
use std::env;

use crate::domain::models::calendar_to_save::CalendarToSave;

pub async fn get_calendar_by_date(
    date: String,
    user_id: String,
) -> Result<Option<CalendarToSave>, mongodb::error::Error> {
    dotenv().ok();

    let connection_string = env::var("CONNECTION_STRING").expect("DATABASE_URL must be set");
    let db_name = "standard_directive";
    let collection_name = "calendars";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<CalendarToSave>(collection_name);

    dbg!(&date.to_string());

    let filter = doc! {"calendar_date": &date.to_string(), "username": user_id};
    let calendar = collection.find_one(filter, None).await?;

    Ok(calendar)
}
