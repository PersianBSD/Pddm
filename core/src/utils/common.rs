use std::error::Error;

/// Trait to abstract disk IO functionality across platforms
pub trait DiskIO {
    /// Create or open a disk device (e.g., "/dev/sda" or "\\.\PhysicalDrive0")
    fn open(path: &str) -> Result<Self, Box<dyn Error>> where Self: Sized;

    /// Read bytes from disk at a specific offset into the given buffer
    fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<usize, Box<dyn Error>>;

    /// Write bytes to disk at a specific offset from the given buffer
    fn write(&mut self, offset: u64, buffer: &[u8]) -> Result<usize, Box<dyn Error>>;

    /// Close the disk handle or resource
    fn close(&mut self);
}
