pub fn to_gb(bytes: u64) -> f32 {
    bytes as f32 / (1024 * 1024 * 1024) as f32
}

pub fn to_mb(bytes: u64) -> f32 {
    bytes as f32 / (1024 * 1024) as f32
}
