//! ابزارهای بررسی صحت داده‌ها مانند CRC یا بررسی اعتبار GUIDها

pub fn is_valid_guid(guid: &str) -> bool {
    // بررسی ساختار پایه GUID با الگوی استاندارد
    let guid_pattern = regex::Regex::new(
        r"^\{?[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\}?$"
    ).unwrap();
    guid_pattern.is_match(guid)
}

/// محاسبه CRC-32 ساده (با استفاده از crate `crc`)
/// در صورت نیاز باید crate آن به `Cargo.toml` افزوده شود:
/// crc = "2.0"
use crc::{Crc, CRC_32_ISO_HDLC};

pub fn compute_crc32(data: &[u8]) -> u32 {
    const CRC32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let mut digest = CRC32.digest();
    digest.update(data);
    digest.finalize()
}
