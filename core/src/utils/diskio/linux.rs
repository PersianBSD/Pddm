// core/src/io/linux.rs

use std::{fs::File, io::{Read, Seek, SeekFrom}, path::Path};

pub struct LinuxDisk {
    pub file: File,
}

impl LinuxDisk {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self { file })
    }

    pub fn read_sector(&mut self, buffer: &mut [u8], offset: u64) -> std::io::Result<usize> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read(buffer)
    }
}

// Drop not required, File auto-closes
