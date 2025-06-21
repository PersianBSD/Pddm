// core/src/disk/os/linux.rs
use crate::disk::DiskInfo;

pub fn list_disks_smart() -> Result<Vec<DiskInfo>, Box<dyn std::error::Error>> {
    Ok(vec![DiskInfo {
        name: "/dev/sda".to_string(),
        size_gb: 500,
        is_removable: false,
        model: Some("Generic Linux Disk".to_string()),
        serial: Some("123456".to_string()),
    }])
}
