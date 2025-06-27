// core/src/utils/platform.rs

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

use std::io::{Result, SeekFrom};

/// رابط عمومی برای دیسک‌های پلتفرم‌های مختلف
pub trait PlatformDisk {
    fn read_sector(&mut self, lba: u64, buffer: &mut [u8]) -> Result<usize>;
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
    fn sector_size(&self) -> u64;
    fn disk_path(&self) -> &str;
}

/// سازنده‌ی دیسک براساس پلتفرم
pub fn open_disk(path: &str) -> Result<Box<dyn PlatformDisk>> {
    #[cfg(target_os = "windows")]
    return windows::WindowsDisk::open(path).map(|d| Box::new(d) as Box<dyn PlatformDisk>);

    #[cfg(target_os = "linux")]
    return linux::LinuxDisk::open(path).map(|d| Box::new(d) as Box<dyn PlatformDisk>);

    #[cfg(target_os = "macos")]
    return macos::MacDisk::open(path).map(|d| Box::new(d) as Box<dyn PlatformDisk>);

    #[allow(unreachable_code)]
    Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Unsupported OS"))
}
