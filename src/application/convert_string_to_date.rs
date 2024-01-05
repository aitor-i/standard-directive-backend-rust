use chrono::{NaiveDateTime, ParseError};

pub fn convert_string_to_date(date_in_string_format: &String) -> Result<NaiveDateTime, ParseError> {
    let format = "%Y-%m-%d %H:%M:%S";
    let naive_date = NaiveDateTime::parse_from_str(&date_in_string_format, format);

    return naive_date;
}
