// مسیر: core/src/util/types/gpt.rs
//! ساختارهای مرتبط با GPT - GUID Partition Table

//use crate::utils::guid::{self, Guid};
use crate::utils::guid::Guid;

/// هدر GPT (در ابتدای دیسک GPT ذخیره می‌شود)
#[derive(Debug, Clone)]
pub struct GptHeader {
    pub signature: [u8; 8],             // امضای "EFI PART"
    pub revision: u32,                 // نسخه GPT
    pub header_size: u32,
    pub crc32: u32,                    // CRC32 هدر
    pub reserved: u32,
    pub current_lba: u64,             // مکان فعلی هدر
    pub backup_lba: u64,              // مکان بکاپ هدر
    pub first_usable_lba: u64,
    pub last_usable_lba: u64,
    pub disk_guid: Guid,         // GUID دیسک
    pub partition_entry_lba: u64,     // شروع جدول پارتیشن‌ها
    pub num_entries: u32,             // تعداد ورودی‌ها
    pub entry_size: u32,              // اندازه هر ورودی
    pub partition_array_crc32: u32,   // CRC32 جدول پارتیشن‌ها
}

/// یک ورودی پارتیشن در جدول GPT
#[derive(Debug, Clone)]
pub struct GptPartitionEntry {
    pub partition_type_guid: [u8; 16],  // نوع پارتیشن (GUID)
    pub unique_partition_guid: [u8; 16],
    pub starting_lba: u64,
    pub ending_lba: u64,
    pub attributes: u64,
    pub partition_name: [u16; 36],     // یونیکد UTF-16
}

/// نوع پارتیشن GPT بر اساس GUID (می‌تواند شناخته‌شده یا ناشناخته باشد)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GptPartitionType {
    /// پارتیشن EFI System
    EfiSystem,
    /// Microsoft Basic Data (NTFS/ExFAT/...) 
    MicrosoftBasic,
    /// Linux Filesystem Data
    LinuxFs,
    /// Linux Swap
    LinuxSwap,
    /// Reserved یا ناشناخته
    Unknown(String),
}

/// اطلاعات پایه دیسک GPT
#[derive(Debug, Clone)]
pub struct GptDiskId {
    pub guid: [u8; 16],
}
