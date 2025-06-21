// core/src/disk/os/mac.rs
use crate::disk::DiskInfo;

pub fn list_disks_smart() -> Result<Vec<DiskInfo>, Box<dyn std::error::Error>> {
    Ok(vec![
        DiskInfo {
            name: "/dev/disk0".to_string(),
            size_gb: 512,
            is_removable: false,
            model: Some("Apple SSD".to_string()),
            serial: Some("MACDISK1234".to_string()),
        }
    ])
}
