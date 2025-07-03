// مسیر: core\src\utils\convert.rs

//! ابزارهای تبدیل برای واحدهای اندازه، اعداد و کار با سکتورها

/// تعریف واحدهای ثابت
const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;
const TB: u64 = GB * 1024;

/// تبدیل بایت به ترابایت با دقت اعشاری 
pub fn bytes_to_tb(bytes: u64) -> f64 {
    (bytes as f64) / TB as f64
}

/// تبدیل بایت به گیگابایت با دقت اعشاری
pub fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64) / GB as f64
}

/// تبدیل بایت به مگابایت با دقت اعشاری
pub fn bytes_to_mb(bytes: u64) -> f64 {
    (bytes as f64) / MB as f64
}

/// تبدیل بایت به کیلوبایت با دقت اعشاری
pub fn bytes_to_kb(bytes: u64) -> f64 {
    (bytes as f64) / KB as f64
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

/// نمایش حجم به‌صورت رشته قابل خواندن (TB، GB، MB و ...)
pub fn format_bytes(bytes: u64) -> String {
    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// قالب‌بندی عدد با جداکننده هزارگان (مثل 1,000,000)
pub fn format_with_separator(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    let chars: Vec<char> = s.chars().rev().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(*ch);
    }

    out.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_tb() {
        assert_eq!(bytes_to_tb(1_099_511_627_776), 1.0);
    }

    #[test]
    fn test_bytes_to_gb() {
        assert_eq!(bytes_to_gb(1_073_741_824), 1.0);
    }

    #[test]
    fn test_bytes_to_mb() {
        assert_eq!(bytes_to_mb(1_048_576), 1.0);
    }

    #[test]
    fn test_bytes_to_kb() {
        assert_eq!(bytes_to_kb(1024), 1.0);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1_099_511_627_776), "1.00 TB");
        assert_eq!(format_bytes(1_073_741_824), "1.00 GB");
        assert_eq!(format_bytes(1_048_576), "1.00 MB");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn test_format_with_separator() {
        assert_eq!(format_with_separator(1000000), "1,000,000");
        assert_eq!(format_with_separator(123456789), "123,456,789");
    }
}