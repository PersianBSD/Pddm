use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Result};
use std::path::Path;

pub struct DiskIO {
    file: File,
}

impl DiskIO {
    pub fn open(disk_path: &str) -> Result<Self> {
        let file = File::open(Path::new(disk_path))?;
        Ok(Self { file })
    }

    pub fn read_sector(&mut self, lba: u64, sector_size: usize) -> Result<Vec<u8>> {
        let offset = lba * sector_size as u64;
        self.file.seek(SeekFrom::Start(offset))?;

        let mut buffer = vec![0u8; sector_size];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}
