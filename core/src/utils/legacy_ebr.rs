use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

#[derive(Debug, Clone)]
pub struct CHSAddress {
    pub head: u8,
    pub sector: u8,
    pub cylinder: u8,
}

#[derive(Debug, Clone)]
pub struct PartitionRecord {
    pub status: u8,
    pub chs_start: CHSAddress,
    pub partition_type: u8,
    pub chs_end: CHSAddress,
    pub begin_lba: u32,
    pub lba_blocks: u32,
}

impl PartitionRecord {
    pub fn new_empty() -> Self {
        Self {
            status: 0,
            chs_start: CHSAddress { head: 0, sector: 0, cylinder: 0 },
            partition_type: 0,
            chs_end: CHSAddress { head: 0, sector: 0, cylinder: 0 },
            begin_lba: 0,
            lba_blocks: 0,
        }
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0] = self.status;
        bytes[1] = self.chs_start.head;
        bytes[2] = self.chs_start.sector;
        bytes[3] = self.chs_start.cylinder;
        bytes[4] = self.partition_type;
        bytes[5] = self.chs_end.head;
        bytes[6] = self.chs_end.sector;
        bytes[7] = self.chs_end.cylinder;
        bytes[8..12].copy_from_slice(&self.begin_lba.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.lba_blocks.to_le_bytes());
        bytes
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            status: data[0],
            chs_start: CHSAddress { head: data[1], sector: data[2], cylinder: data[3] },
            partition_type: data[4],
            chs_end: CHSAddress { head: data[5], sector: data[6], cylinder: data[7] },
            begin_lba: u32::from_le_bytes(data[8..12].try_into().unwrap()),
            lba_blocks: u32::from_le_bytes(data[12..16].try_into().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EbrSector {
    pub boot_code: [u8; 446],
    pub entries: [PartitionRecord; 2],
    pub signature: [u8; 2],
}

impl Default for EbrSector {
    fn default() -> Self {
        Self {
            boot_code: [0u8; 446],
            entries: [PartitionRecord::new_empty(), PartitionRecord::new_empty()],
            signature: [0x55, 0xAA],
        }
    }
}

pub struct EbrChainWriter<'a> {
    pub disk: &'a mut File,
    pub logical_partitions: Vec<PartitionRecord>,
    pub start_lba: u64,
}

impl<'a> EbrChainWriter<'a> {
    pub fn new(disk: &'a mut File) -> Self {
        Self {
            disk,
            logical_partitions: vec![],
            start_lba: 0,
        }
    }

    pub fn write_chain(&mut self) -> std::io::Result<()> {
        for i in 0..self.logical_partitions.len() {
            let mut sector = [0u8; 512];
            let (entry1_bytes, entry2_bytes);

            let entry1 = &self.logical_partitions[i];
            let relative_lba = entry1.begin_lba as u64 - self.start_lba;

            let mut entry1_new = PartitionRecord::new_empty();
            entry1_new.status = 0x00;
            entry1_new.partition_type = entry1.partition_type;
            entry1_new.begin_lba = relative_lba as u32;
            entry1_new.lba_blocks = entry1.lba_blocks;
            entry1_bytes = entry1_new.to_bytes();
            sector[446..462].copy_from_slice(&entry1_bytes);

            if i + 1 < self.logical_partitions.len() {
                let next = &self.logical_partitions[i + 1];
                let rel_next = next.begin_lba as u64 - self.start_lba;
                let mut entry2 = PartitionRecord::new_empty();
                entry2.status = 0x00;
                entry2.partition_type = 0x05;
                entry2.begin_lba = rel_next as u32;
                entry2.lba_blocks = next.lba_blocks;
                entry2_bytes = entry2.to_bytes();
                sector[462..478].copy_from_slice(&entry2_bytes);
            }

            sector[510] = 0x55;
            sector[511] = 0xAA;

            let abs_lba = if i == 0 {
                self.start_lba
            } else {
                self.logical_partitions[i].begin_lba as u64
            };

            self.disk.seek(SeekFrom::Start(abs_lba * 512))?;
            self.disk.write_all(&sector)?;
        }

        Ok(())
    }
}
