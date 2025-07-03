// مسیر: core/src/util/types/ebr.rs
//! ساختارهای مربوط به EBR (Extended Boot Record)

use super::mbr::MbrPartitionEntry;

/// ساختار سکتور EBR - مانند MBR ولی فقط شامل دو ورودی پارتیشن است
#[derive(Debug, Clone)]
pub struct EbrSector {
    pub boot_code: [u8; 446],
    pub entries: [MbrPartitionEntry; 2],
    pub signature: u16,
}

impl Default for EbrSector {
    fn default() -> Self {
        EbrSector {
            boot_code: [0; 446],
            entries: [MbrPartitionEntry {
                status: 0,
                chs_start: [0; 3],
                partition_type: 0,
                chs_end: [0; 3],
                begin_lba: 0,
                lba_blocks: 0,
            }; 2],
            signature: 0xAA55,
        }
    }
}
