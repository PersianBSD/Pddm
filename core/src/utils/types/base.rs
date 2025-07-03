//core/src/utils/types/base.rs
//! ساختارهای پایه برای عملیات مشترک و هسته‌ای دیسک و پارتیشن‌ها



/// فضای آزاد در دیسک یا پارتیشن
#[derive(Debug, Clone)]
pub enum FreeSpaceType {
    Unallocated,
    Extended,
    Gpt,
}

/// نمایش فضای آزاد بین پارتیشن‌ها
#[derive(Debug, Clone)]
pub struct FreeSpace {
    pub begin_sector: u64,
    pub length: u64,
    pub space_type: FreeSpaceType,
}

/// پارتیشن عمومی برای تحلیل ساختارهای مختلف
#[derive(Debug, Clone)]
pub struct GenPart {
    pub start_sector: u64,
    pub total_sectors: u64,
    pub part_type_code: u8,
    pub gpt_type_guid: Option<String>,
}

impl GenPart {
    /// بررسی اینکه پارتیشن از نوع logical (در MBR)
    pub fn is_logical(&self) -> bool {
        matches!(self.part_type_code, 0x05 | 0x0F | 0x85)
    }

    /// بررسی اینکه پارتیشن از نوع extended (در MBR)
    pub fn is_extended(&self) -> bool {
        matches!(self.part_type_code, 0x05 | 0x0F)
    }

    /// بررسی اینکه پارتیشن GPT است
    pub fn is_gpt(&self) -> bool {
        self.gpt_type_guid.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_type_check() {
        let p = GenPart {
            start_sector: 2048,
            total_sectors: 4096,
            part_type_code: 0x05,
            gpt_type_guid: None,
        };

        assert!(p.is_logical());
        assert!(p.is_extended());
        assert!(!p.is_gpt());
    }
}
