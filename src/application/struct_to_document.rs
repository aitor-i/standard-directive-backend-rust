use bson::{ser::Error as BsonError, to_bson, Bson, Document};
use serde::{ser::Error as SerError, Serialize};

pub fn struct_to_document<T: Serialize>(data: &T) -> Result<Document, BsonError> {
    match to_bson(data) {
        Ok(Bson::Document(document)) => Ok(document),
        Ok(_) => Err(BsonError::custom("Expected a BSON document.")),
        Err(e) => Err(e),
    }
}
