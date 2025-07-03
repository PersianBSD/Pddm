//! انواع پایه برای مدیریت دیسک، پارتیشن، GPT، فضای خالی و ...




/// هدر جدول پارتیشن GPT
#[derive(Debug, Clone)]
pub struct GptHeader {
    pub signature: [u8; 8],          // "EFI PART"
    pub revision: u32,
    pub header_size: u32,
    pub header_crc32: u32,
    pub reserved: u32,
    pub current_lba: u64,
    pub backup_lba: u64,
    pub first_usable_lba: u64,
    pub last_usable_lba: u64,
    pub disk_guid: [u8; 16],
    pub partition_entries_lba: u64,
    pub num_partition_entries: u32,
    pub size_of_partition_entry: u32,
    pub partition_entries_crc32: u32,
}

/// ساختار هر ورودی جدول پارتیشن GPT
#[derive(Debug, Clone)]
pub struct GptPartitionEntry {
    pub partition_type_guid: [u8; 16],
    pub unique_partition_guid: [u8; 16],
    pub starting_lba: u64,
    pub ending_lba: u64,
    pub attributes: u64,
    pub partition_name: String, // یونیکد در GPT
}

// ------------------------
// تست اولیه برای ساختارها
// ------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_part_gpt_detection() {
        let part = GenPart {
            start_sector: 2048,
            total_sectors: 409600,
            part_type_code: 0x07,
            gpt_type_guid: Some("EBD0A0A2-B9E5-4433-87C0-68B6B72699C7".into()),
        };
        assert!(part.is_gpt());
        assert!(!part.is_extended());
        assert!(!part.is_logical());
    }

    #[test]
    fn test_partition_style_enum() {
        let style = PartitionStyle::GPT;
        assert_eq!(style, PartitionStyle::GPT);
    }
}
