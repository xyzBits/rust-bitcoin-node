use std::time::SystemTime;

pub fn now() -> u64 {
    use std::time::UNIX_EPOCH;
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn ms_since(start: &SystemTime) -> f32 {
    start.elapsed().unwrap().as_secs_f32() * 1000.0
}
