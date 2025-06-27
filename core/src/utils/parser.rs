// src/utils/parser.rs

use crate::utils::types::{PartitionStyle, PartitionType};

/// Parses a string and returns the corresponding PartitionStyle if known.
pub fn parse_partition_style(s: &str) -> Option<PartitionStyle> {
    match s.to_lowercase().as_str() {
        "mbr" => Some(PartitionStyle::MBR),
        "gpt" => Some(PartitionStyle::GPT),
        _ => None,
    }
}

/// Parses a string and returns a simplified PartitionType.
pub fn parse_partition_type(s: &str) -> PartitionType {
    let lower = s.to_lowercase();
    if lower.contains("efi") || lower.contains("esp") {
        PartitionType::EFI
    } else if lower.contains("swap") {
        PartitionType::Swap
    } else if lower.contains("ntfs") {
        PartitionType::NTFS
    } else if lower.contains("fat") {
        PartitionType::FAT
    } else if lower.contains("ext") {
        PartitionType::EXT
    } else if lower.contains("linux") {
        PartitionType::Linux
    } else if lower.contains("windows") {
        PartitionType::Windows
    } else if lower.contains("reserved") || lower.contains("msr") {
        PartitionType::Reserved
    } else {
        PartitionType::Other
    }
}
