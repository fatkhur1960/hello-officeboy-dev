//! Utilitas yang berkaitan dengan waktu,
//! di sini kita bisa mendapatkan waktu terkini dalam milidetik dll.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Mendapatkan waktu saat ini dalam format milidetik sejak UNIX EPOCH.
pub fn current_time_millis() -> u64 {
    current_time().as_millis() as u64
}

/// Mendapatkan waktu saat ini dalam format milidetik sejak UNIX EPOCH.
pub fn current_time() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
}
