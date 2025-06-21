use std::error::Error;
use crate::disk::os::windows::list_disks_smart;

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub size_gb: u64,
    pub is_removable: bool,
    pub model: Option<String>,
    pub serial: Option<String>,
}

pub fn smart_list_disks() -> Result<Vec<DiskInfo>, Box<dyn Error>> {
    let disks = list_disks_smart()?; // فقط تابع جامع که ترکیب WMI و DeviceIo است
    Ok(disks)
}
