use chrono::Utc;

pub fn get_timestamp() -> String {
    Utc::now().to_rfc3339()
}
