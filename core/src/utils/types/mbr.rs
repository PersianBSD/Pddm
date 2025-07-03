// مسیر: core/src/util/types/mbr.rs
//! ساختارها و تعاریف مربوط به MBR (Master Boot Record)

/// ساختار رکورد پارتیشن در MBR - شامل 16 بایت اطلاعات
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MbrPartitionEntry {
    pub status: u8,
    pub chs_start: [u8; 3],
    pub partition_type: u8,
    pub chs_end: [u8; 3],
    pub begin_lba: u32,
    pub lba_blocks: u32,
}

impl MbrPartitionEntry {
    /// بررسی خالی بودن رکورد پارتیشن
    pub fn is_empty(&self) -> bool {
        self.partition_type == 0
    }
}

/// ساختار اصلی سکتور MBR - شامل 512 بایت
#[derive(Debug, Clone)]
pub struct MbrSector {
    pub boot_code: [u8; 446],
    pub entries: [MbrPartitionEntry; 4],
    pub signature: u16,
}

impl Default for MbrSector {
    fn default() -> Self {
        MbrSector {
            boot_code: [0; 446],
            entries: [MbrPartitionEntry {
                status: 0,
                chs_start: [0; 3],
                partition_type: 0,
                chs_end: [0; 3],
                begin_lba: 0,
                lba_blocks: 0,
            }; 4],
            signature: 0xAA55,
        }
    }
}
