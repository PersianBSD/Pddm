// فایل util/types.rs

/// نوع پارتیشن: MBR یا GPT
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartitionStyle {
    MBR,
    GPT,
    Unknown,
}

/// ساختار اطلاعات دیسک
#[derive(Debug, Clone)]
pub struct DiskBasicInfo {
    pub name: String,
    pub size_bytes: u64,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub style: PartitionStyle,
    pub is_removable: bool,
}

/// ساختار اطلاعات پارتیشن
#[derive(Debug, Clone)]
pub struct PartitionBasicInfo {
    pub mount_point: Option<String>,
    pub file_system: Option<String>,
    pub total_space: Option<u64>,
    pub used_space: Option<u64>,
    pub free_space: Option<u64>,
    pub label: Option<String>,
    pub guid: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FreeSpaceType {
    Unallocated,
    Extended,
    Gpt,
}

#[derive(Debug, Clone)]
pub struct FreeSpace {
    pub begin_sector: u64,
    pub length: u64,
    pub space_type: FreeSpaceType,
}

#[derive(Debug, Clone)]
pub struct GenPart {
    pub start_sector: u64,
    pub total_sectors: u64,
    pub part_type_code: u8,
    pub gpt_type_guid: Option<String>,
}

impl GenPart {
    pub fn is_logical(&self) -> bool {
        self.part_type_code == 0x05 || self.part_type_code == 0x0F || self.part_type_code == 0x85
    }

    pub fn is_extended(&self) -> bool {
        self.part_type_code == 0x05 || self.part_type_code == 0x0F
    }

    pub fn is_gpt(&self) -> bool {
        self.gpt_type_guid.is_some()
    }
}
