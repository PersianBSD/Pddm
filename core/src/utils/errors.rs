use std::fmt;

/// خطاهای مرتبط با عملیات دیسک
#[derive(Debug)]
pub enum DiskError {
    OpenDisk,
    CreateFile,
    WriteFailure,
    PartitionTooSmall,
    GptAlreadyExists,
    BpsNotPowerOfTwo,
    PartitionsOverlap,
    VolumeNotMounted,
    IoError(String),
    Custom(String),
}

impl fmt::Display for DiskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DiskError::*;
        let message = match self {
            OpenDisk => "❌ Could not open disk",
            CreateFile => "❌ Could not create file",
            WriteFailure => "❌ Write to disk failed",
            PartitionTooSmall => "❌ Partition is too small",
            GptAlreadyExists => "❌ GPT already exists",
            BpsNotPowerOfTwo => "❌ Bytes per sector must be power of 2",
            PartitionsOverlap => "❌ Partitions overlap",
            VolumeNotMounted => "❌ Volume is not mounted",
            IoError(msg) => &msg,
            Custom(msg) => &msg,
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for DiskError {}

/// کمک کننده برای تبدیل از std::io::Error به DiskError
impl From<std::io::Error> for DiskError {
    fn from(err: std::io::Error) -> Self {
        DiskError::IoError(err.to_string())
    }
}
