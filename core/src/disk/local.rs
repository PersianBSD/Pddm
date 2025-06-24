#[cfg(target_os = "windows")]
mod os_impl {
       pub use crate::disk::os::windows::smart_disks_list;
}

#[cfg(target_os = "linux")]
mod os_impl {
        pub use crate::disk::os::linux::smart_disks_list;
}

#[cfg(target_os = "macos")]
mod os_impl {
    pub use crate::disk::os::mac::*;
}

pub struct LocalDiskProvider;

use crate::disk::provider::DiskInfo;

pub fn get_disks() -> Vec<DiskInfo> {
    match os_impl::smart_disks_list() {
        Ok(disks) => disks,
        Err(e) => {
            eprintln!("خطا در تشخیص دیسک‌ها: {}", e);
            vec![]
        }
    }
}


