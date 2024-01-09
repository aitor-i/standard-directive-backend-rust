use chrono::NaiveDateTime;
use mongodb::bson::doc;
use mongodb::Client;

use crate::domain::models::calendar_to_save::CalendarToSave;

pub async fn get_calendar_by_date(
    date: NaiveDateTime,
) -> Result<Option<CalendarToSave>, mongodb::error::Error> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "calendars";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection = database.collection::<CalendarToSave>(collection_name);

    dbg!(&date.to_string());

    let filter = doc! {"calendar_date": &date.to_string()};
    let calendar = collection.find_one(filter, None).await?;

    Ok(calendar)
}
