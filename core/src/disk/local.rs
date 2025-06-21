#[cfg(target_os = "windows")]
mod os_impl {
       pub use crate::disk::os::windows::list_disks_smart;
}

#[cfg(target_os = "linux")]
mod os_impl {
        pub use crate::disk::os::linux::list_disks_smart;
}

#[cfg(target_os = "macos")]
mod os_impl {
    pub use crate::disk::os::mac::*;
}

pub struct LocalDiskProvider;

use crate::disk::provider::DiskInfo;

pub fn get_disks() -> Vec<DiskInfo> {
    match os_impl::list_disks_smart() {
        Ok(disks) => disks,
        Err(e) => {
            eprintln!("خطا در تشخیص دیسک‌ها: {}", e);
            vec![]
        }
    }
}


