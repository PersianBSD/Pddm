// مسیر: core/src/util/types/fsid.rs

//! ساختارهای مرتبط با شناسه فایل‌سیستم‌ها (FSID) برای شناسایی نوع پارتیشن‌ها و فایل‌سیستم

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FsidEntry {
    /// نام فایل‌سیستم (مثلاً: NTFS, ext4, FAT32)
    pub name: String,
    /// شناسه GUID برای GPT یا FSID برای MBR
    pub guid: String,
    /// نوع پارتیشن یا سیستم‌عامل مربوط به این FSID
    pub system_type: Option<String>,
    /// توضیحات اضافی در صورت نیاز
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsidFormat {
    /// سیستم MBR با شناسه عددی (بایت واحد)
    Mbr(u8),
    /// سیستم GPT با شناسه GUID
    Gpt(String),
    /// ناشناس یا پشتیبانی‌نشده
    Unknown,
}
