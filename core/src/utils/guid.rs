// GUID utilities module

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guid(pub String);

impl Guid {
    pub fn new(guid: &str) -> Self {
        Self(guid.to_ascii_lowercase())
    }

    pub fn is_equal(&self, other: &Guid) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }

    pub fn is_known_partition_type(&self) -> bool {
        matches!(
            self.0.as_str(),
            "{c12a7328-f81f-11d2-ba4b-00a0c93ec93b}" // EFI System Partition
            | "{ebd0a0a2-b9e5-4433-87c0-68b6b72699c7}" // Basic Data Partition
            | "{de94bba4-06d1-4d40-a16a-bfd50179d6ac}" // Windows Recovery Environment
            | "{e3c9e316-0b5c-4db8-817d-f92df00215ae}" // Microsoft Reserved Partition
            | "{0657fd6d-a4ab-43c4-84e5-0933c84b4f4f}" // Linux Swap
            | "{0fc63daf-8483-4772-8e79-3d69d8477de4}"   // Linux Filesystem
        )
    }
}

// Optional display formatting
impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
} 
