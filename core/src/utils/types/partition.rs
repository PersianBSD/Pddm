// مسیر: core/src/utils/types/partition.rs

use crate::utils::types::gpt::GptPartitionEntry;
use crate::utils::types::mbr::MbrPartitionEntry;

/// نوع ساختار پارتیشن
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartitionKind {
    Mbr(MbrPartitionEntry),
    Gpt(GptPartitionEntry),
    Unknown,
}

/// پارتیشن عمومی مستقل از سبک MBR/GPT
#[derive(Debug, Clone)]
pub struct GenPart {
    pub start_sector: u64,
    pub total_sectors: u64,
    pub kind: PartitionKind,
}

impl GenPart {
    pub fn is_logical(&self) -> bool {
        matches!(self.kind, PartitionKind::Mbr(ref mbr) if mbr.partition_type == 0x05 || mbr.partition_type == 0x0F || mbr.partition_type == 0x85)
    }

    pub fn is_extended(&self) -> bool {
        matches!(self.kind, PartitionKind::Mbr(ref mbr) if mbr.partition_type == 0x05 || mbr.partition_type == 0x0F)
    }

    pub fn is_gpt(&self) -> bool {
        matches!(self.kind, PartitionKind::Gpt(_))
    }
}
