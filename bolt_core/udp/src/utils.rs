use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    since_epoch.as_millis() as u64
}
