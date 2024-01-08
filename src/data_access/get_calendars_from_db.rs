use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CalendarModel {
    calendar_id: String,
}

pub async fn get_calendars_from_db() -> Result<(), mongodb::error::Error> {
    let connection_string = "mongodb://localhost:27017";
    let db_name = "standard_directive";
    let collection_name = "calendars";

    let client = Client::with_uri_str(connection_string).await?;
    let database = client.database(db_name);
    let collection: Collection<Document> = database.collection(collection_name);

    let filter = doc! {};
    let cursor = collection.find(filter, None).await;

    match cursor {
        Ok(mongo_document) => {
            let bson_raw = mongo_document.current();
            let raw_buf = bson_raw.to_raw_document_buf();
            dbg!(raw_buf);
            // dbg!(bson_documet);
        }
        Err(_) => {
            println!("Erro on formating");
        }
    }

    Ok(())

    // let mut documents: Vec<Document> = Vec::new();
    // while let Some(result) = cursor.await {
    //     match result {
    //         Ok(document) => documents.push(document),
    //         Err(e) => return Err(e.into()), // Handle the error
    //     }
    // }

    // let document = match cursor.await {
    //     Ok(document_cursor) => {
    //         dbg!(&document_cursor);
    //         mongodb::bson::from_bson(bson::Bson::Document(document_cursor[0]))
    //     }
    //     Err(err) => {
    //         println!("Error");
    //         err
    //     }
    //     _ => {
    //         println!("None");
    //     }
    // };

    // for calendar in cursor {
    //     match calendar {
    //         Ok(document) => {
    //             match bson::from_bson<CalendarModel>(bson::Bson::Document(document)) {
    //                 Ok(calendar_model) => {calendar_model}
    //                 Err(e) => {e}

    //             }
    //         }
    //         Err(err) => {err}

    //     }
    //     Ok(calendar);
    // }
}
