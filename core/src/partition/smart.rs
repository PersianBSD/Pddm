// core/src/disk/smart.rs
//use crate::disk::provider::DiskInfo;

#[cfg(target_os = "windows")]
mod platform {
    pub use crate::partition::os::windows::smart_partition_list;
}

#[cfg(target_os = "linux")]
mod platform {
   pub use crate::partition::os::linux::smart_partition_list;
}
/*
#[cfg(target_os = "macosx")]
mod platform {
    pub use crate::partition::os::macos::smart_partition_list;
}
*/
pub use platform::smart_partition_list;

