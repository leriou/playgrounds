use time::OffsetDateTime;

pub fn now_unix_timestamp() -> i64 {
    OffsetDateTime::now_utc().unix_timestamp()
}

pub fn now_nanos() -> i128 {
    OffsetDateTime::now_utc().unix_timestamp_nanos()
}