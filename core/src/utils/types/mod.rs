// core/src/utils/types/mod.rs

//! ماژول اصلی انواع داده (Types)
//! این فایل، زیرماژول‌های مختلف را برای ساختارهای دیسک، پارتیشن، GPT، MBR و ... گردآوری می‌کند

pub mod base;        // FreeSpace, GenPart, ...
pub mod info;        // DiskBasicInfo, PartitionBasicInfo, PartitionStyle
pub mod gpt;         // GptHeader, GptPartitionEntry, GUIDs for GPT
//pub mod legacy;      // MBR / EBR structs like PartitionRecord
pub mod guid;        // GUID string handling, known GUID types

// re-export پرکاربردها برای سادگی استفاده از خارج ماژول
//pub use core::{FreeSpace, FreeSpaceType, GenPart};
//pub use info::{DiskBasicInfo, PartitionBasicInfo, PartitionStyle};
pub use gpt::{GptHeader, GptPartitionEntry};
//pub use legacy::PartitionRecord;
//pub use guid::KnownGuidType;
