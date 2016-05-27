
extern crate chrono;
use chrono::*;

pub fn after(datetime: DateTime<UTC>) -> DateTime<UTC> {
    datetime + Duration::seconds(1_000_000_000)
}
