//! GPT structures and functions extracted from original `gpt.h`/`gpt.cpp`
//! This module provides parsing structures for GUID Partition Table (GPT)

use uuid::Uuid;
use std::fmt;
use crate::utils::types::{GenPart, GUID};
use crate::utils::crc::Crc32;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

impl fmt::Debug for GptPartitionEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name: String = String::from_utf16_lossy(&self.partition_name);
        write!(f, "GPT Entry: LBA {} - {}, Name: {}", self.starting_lba, self.ending_lba, name.trim())
    }
}

/// Utility to convert raw GUID bytes to readable UUID
pub fn parse_guid(bytes: &[u8; 16]) -> Option<Uuid> {
    Uuid::from_slice(bytes).ok()
}

#[derive(Debug, Clone)]
pub struct GptSpecific {
    pub type_guid: GUID,
    pub unique_guid: GUID,
    pub name: [u16; 36],
    pub flags: u64,
}

pub struct GptHelper<'a> {
    pub disk: &'a mut dyn crate::utils::disk_io::DiskIo,
}

impl<'a> GptHelper<'a> {
    pub fn new(disk: &'a mut dyn crate::utils::disk_io::DiskIo) -> Self {
        Self { disk }
    }

    pub fn write_changes(&mut self) {
        // TODO: پیاده‌سازی WriteChanges مشابه C++
    }

    pub fn read_partition_tables(&mut self, gpt_part: &GenPart) {
        // TODO: پیاده‌سازی خواندن جدول
    }

    pub fn create_gpt(&self, gpt_part: &GenPart, entries: u32) -> GptHeader {
        // TODO: پیاده‌سازی ایجاد GPT
        unimplemented!()
    }

    pub fn is_valid_gpt(&self, gpt: &GptHeader) -> bool {
        // TODO: بررسی صحت امضای GPT و CRC32
        unimplemented!()
    }
}

impl GptHeader {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        let mut rdr = Cursor::new(data);
        let mut signature = [0u8; 8];
        rdr.read_exact(&mut signature).ok()?;

        Some(GptHeader {
            signature,
            revision: rdr.read_u32::<LittleEndian>().ok()?,
            header_size: rdr.read_u32::<LittleEndian>().ok()?,
            current_lba: rdr.read_u64::<LittleEndian>().ok()?,
            backup_lba: rdr.read_u64::<LittleEndian>().ok()?,
            first_usable_lba: rdr.read_u64::<LittleEndian>().ok()?,
            last_usable_lba: rdr.read_u64::<LittleEndian>().ok()?,
            disk_guid: read_guid(&mut rdr)?,
            partition_entries_lba: rdr.read_u64::<LittleEndian>().ok()?,
            num_partition_entries: rdr.read_u32::<LittleEndian>().ok()?,
            size_of_partition_entry: rdr.read_u32::<LittleEndian>().ok()?,
        })
    }
}


impl GptPartitionEntry {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        let mut rdr = Cursor::new(data);

        Some(GptPartitionEntry {
            partition_type_guid: read_guid(&mut rdr)?,
            unique_partition_guid: read_guid(&mut rdr)?,
            starting_lba: rdr.read_u64::<LittleEndian>().ok()?,
            ending_lba: rdr.read_u64::<LittleEndian>().ok()?,
            attributes: rdr.read_u64::<LittleEndian>().ok()?,
            partition_name: read_utf16le_name(&mut rdr, 36),
        })
    }
}

fn read_guid(rdr: &mut Cursor<&[u8]>) -> Option<Uuid> {
    let mut guid_bytes = [0u8; 16];
    rdr.read_exact(&mut guid_bytes).ok()?;
    Uuid::from_slice(&guid_bytes).ok()
}

fn read_utf16le_name(rdr: &mut Cursor<&[u8]>, len: usize) -> String {
    let mut name = String::new();
    for _ in 0..len {
        if let Ok(ch) = rdr.read_u16::<LittleEndian>() {
            if ch == 0 {
                break;
            }
            if let Some(c) = std::char::from_u32(ch as u32) {
                name.push(c);
            }
        }
    }
    name
}
