// core/src/disk/smart.rs
//use crate::disk::provider::DiskInfo;

#[cfg(target_os = "windows")]
mod platform {
    pub use crate::disk::os::windows::smart_disks_list;
}

#[cfg(target_os = "linux")]
mod platform {
    pub use crate::disk::os::linux::smart_disks_list;
}

/* 
#[cfg(target_os = "macosx")]
mod platform {
    pub use crate::disk::os::macos::smart_disks_list;
}
*/
pub use platform::smart_disks_list;
