// مسیر: core/src/lib/convert.rs

/// تبدیل بایت به گیگابایت (به صورت اعشاری)
pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_000_000_000.0
}

/// تبدیل بایت به مگابایت (به صورت اعشاری)
pub fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / 1_000_000.0
}
