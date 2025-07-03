/// اطلاعات پایه درباره‌ی دیسک
#[derive(Debug, Clone)]
pub struct DiskBasicInfo {
    pub name: String,                   // نام دیسک (مثلاً \\.\PhysicalDrive0)
    pub size_bytes: u64,               // اندازه کل به بایت
    pub model: Option<String>,         // مدل دیسک
    pub serial: Option<String>,        // شماره سریال دیسک
    pub style: PartitionStyle,         // سبک پارتیشن‌بندی (MBR یا GPT)
    pub is_removable: bool,            // قابل جداسازی بودن (مثلاً USB)
}


/// اطلاعات پایه درباره‌ی پارتیشن
#[derive(Debug, Clone)]
pub struct PartitionBasicInfo {
    pub mount_point: Option<String>,   // نقطه اتصال (مانت)
    pub file_system: Option<String>,   // نوع فایل‌سیستم
    pub total_space: Option<u64>,      // کل فضا (بایت)
    pub used_space: Option<u64>,       // فضای مصرف‌شده
    pub free_space: Option<u64>,       // فضای آزاد
    pub label: Option<String>,         // برچسب (نام) پارتیشن
    pub guid: Option<String>,          // شناسه GUID در GPT
}

/// نوع پارتیشن: MBR یا GPT یا ناشناخته
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartitionStyle {
    MBR,
    GPT,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub disk_name: String,
    pub size_gb: u64,
    pub is_removable: bool,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub partition_style : Option<String>,
    pub bus_type: Option<String>,       
    pub media_type: Option<String>, 
}

#[derive(Debug, Clone)]
pub struct PartitionInfo {
    pub partition_name: String,
    pub mount_point: Option<String>,
    pub file_system: Option<String>,
    pub total_space: Option<u64>,
    pub used_space: Option<u64>,
    pub free_space: Option<u64>,
    pub volume_label: Option<String>,
    pub volume_id: Option<String>,

    // اطلاعات تکمیلی
    pub disk_number: Option<u32>,
    pub partition_number: Option<u32>,
    pub offset: Option<u64>,
    pub media_type: Option<String>,
    pub bus_type: Option<String>,
    pub is_boot: Option<bool>,
    pub is_system: Option<bool>,
    pub is_hidden: Option<bool>,
    pub guid_type: Option<String>,
    pub partition_type: Option<String>,
}