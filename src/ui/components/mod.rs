use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_system_time(value: SystemTime) -> String {
    match value.duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}s", duration.as_secs()),
        Err(_) => "desconhecido".to_owned(),
    }
}
