// core/src/disk/smart.rs
use crate::disk::DiskInfo;

#[cfg(target_os = "windows")]
mod platform {
    pub use crate::disk::os::windows::list_disks_smart;
}

#[cfg(target_os = "linux")]
mod platform {
    pub use crate::disk::os::linux::list_disks_smart;
}

#[cfg(target_os = "macosx")]
mod platform {
    pub use crate::disk::os::linux::list_disks_smart;
}

pub use platform::list_disks_smart;
