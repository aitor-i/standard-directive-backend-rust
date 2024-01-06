use bson::{ser::Error, to_bson, Bson, Document};
use serde::{ser::Error as SerError, Serialize};

pub fn struct_to_document<T: Serialize>(data: &T) -> Result<Document, Error> {
    match to_bson(data) {
        Ok(Bson::Document(document)) => Ok(document),
        Ok(_) => Err(Error::custom("Expected a BSON document.")),
        Err(e) => Err(e),
    }
}
