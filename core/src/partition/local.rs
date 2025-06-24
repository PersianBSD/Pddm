#[cfg(target_os = "windows")]
mod os_impl {
       pub use crate::partition::os::windows::smart_partition_list;
}

#[cfg(target_os = "linux")]
mod os_impl {
        pub use crate::partition::os::linux::smart_partition_list;
}

#[cfg(target_os = "macos")]
mod os_impl {
    pub use crate::partition::os::mac::*;
}

pub struct LocalPartitionProvider;

use crate::partition::provider::PartitionInfo;

pub fn get_partitions() -> Vec<PartitionInfo> {
    match os_impl::smart_partition_list() {
        Ok(partitions) => partitions,
        Err(e) => {
            eprintln!("❌ خطا در واکشی پارتیشن‌ها: {}", e);
            vec![]
        }
    }
}
