// مسیر: core/src/util/types/volume.rs

/// ساختار اطلاعات مربوط به لیبل، سیستم‌فایل، MountPoint و GUID volume
#[derive(Debug, Clone)]
pub struct VolumeInfo {
    /// برچسب (Label) اختصاصی برای پارتیشن یا درایو
    pub label: Option<String>,

    /// نوع سیستم‌فایل (مثلاً NTFS, FAT32, ext4, ...)
    pub file_system: Option<String>,

    /// نقطه‌ی Mount پارتیشن یا درایو (مثلاً C:\، /mnt/data و ...)
    pub mount_point: Option<String>,

    /// شناسه‌ی یکتای volume (GUID)
    pub guid: Option<String>,

    /// حجم کلی به بایت
    pub total_space: Option<u64>,

    /// فضای آزاد به بایت
    pub free_space: Option<u64>,

    /// فضای مصرف‌شده به بایت
    pub used_space: Option<u64>,
}

impl VolumeInfo {
    /// محاسبه حجم استفاده‌شده در صورت امکان
    pub fn calculate_used(&mut self) {
        if let (Some(total), Some(free)) = (self.total_space, self.free_space) {
            self.used_space = Some(total.saturating_sub(free));
        }
    }
}
