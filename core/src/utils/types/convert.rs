// مسیر: core/src/lib/convert.rs

/// ابزارهای تبدیل برای واحدهای اندازه، اعداد و کار با سکتورها.

/// تبدیل بایت به ترابایت با دقت اعشاری 
pub fn bytes_to_tb(bytes: u64) -> f64 { 
    (bytes as f64) / 1_099_511_627_776.0
}
/// تبدیل بایت به گیگابایت با دقت اعشاری
pub fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64) / 1_073_741_824.0
}

/// تبدیل بایت به مگابایت با دقت اعشاری
pub fn bytes_to_mb(bytes: u64) -> f64 {
    (bytes as f64) / 1_048_576.0
}

/// تبدیل بایت به کیلوبایت با دقت اعشاری
pub fn bytes_to_kb(bytes: u64) -> f64 {
    (bytes as f64) / 1024.0
}

/// تبدیل سکتور و اندازه سکتور به بایت
pub fn sectors_to_bytes(sectors: u64, sector_size: u64) -> u64 {
    sectors * sector_size
}

/// تبدیل بایت به تعداد سکتور با توجه به اندازه سکتور
pub fn bytes_to_sectors(bytes: u64, sector_size: u64) -> u64 {
    if sector_size == 0 {
        0
    } else {
        bytes / sector_size
    }
}

/// نمایش حجم به‌صورت رشته قابل خواندن (GB، MB و ...)
pub fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_099_511_627_776 {
        format!("{:.2} TB", bytes as f64 / 1_099_511_627_776.0)
    } else if bytes >= 1_073_741_824 {
        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.2} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}


/// قالب‌بندی عدد با جداکننده هزارگان (مثل 1,000,000)
pub fn format_with_separator(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    let mut chars = s.chars().rev().collect::<Vec<_>>();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(*ch);
    }

    out.chars().rev().collect()
}
