// core/src/util/disk/manager.rs

use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
use windows::core::PWSTR;
//use std::ptr;

/// تست فضای آزاد روی درایو مشخص شده
pub fn get_disk_free_space(path: &str) -> Option<(u64, u64, u64)> {
    let drive: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
    let mut avail = 0u64;
    let mut total = 0u64;
    let mut free = 0u64;

    let result = unsafe {
        GetDiskFreeSpaceExW(
            PWSTR(drive.as_ptr() as *mut _),
            Some(&mut avail),
            Some(&mut total),
            Some(&mut free),
        )
    };

    if result.is_ok() {
        Some((avail, total, free))
    } else {
        None
    }
}


