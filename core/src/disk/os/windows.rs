use crate::disk::provider::DiskInfo;
use std::collections::HashMap;
use windows::{
    core::PCWSTR,
    Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
    Win32::Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_SHARE_READ,
        FILE_SHARE_WRITE, OPEN_EXISTING,
    },
    Win32::System::Ioctl::{DISK_GEOMETRY_EX, IOCTL_DISK_GET_DRIVE_GEOMETRY_EX},
    Win32::System::IO::DeviceIoControl,
};
use wmi::{COMLibrary, Variant, WMIConnection};

pub fn list_disks_smart() -> Result<Vec<DiskInfo>, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;
    let wmi_results: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT * FROM Win32_DiskDrive")?;

    let mut disks = vec![];

    for i in 0..=10 {
        let path = format!("\\\\.\\PhysicalDrive{}", i);
        let path_w: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

        let handle = unsafe {
            CreateFileW(
                PCWSTR(path_w.as_ptr()),
                FILE_GENERIC_READ.0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        };

        if let Ok(h) = handle {
            if h.0 == INVALID_HANDLE_VALUE.0 {
                continue;
            }

            // گرفتن سایز دیسک
            let mut out_buffer = [0u8; std::mem::size_of::<DISK_GEOMETRY_EX>() + 1024];
            let mut returned = 0u32;
            let size = if unsafe {
                DeviceIoControl(
                    h,
                    IOCTL_DISK_GET_DRIVE_GEOMETRY_EX,
                    None,
                    0,
                    Some(out_buffer.as_mut_ptr() as *mut _),
                    out_buffer.len() as u32,
                    Some(&mut returned),
                    None,
                )
            }
            .is_ok()
            {
                let geometry = unsafe { &*(out_buffer.as_ptr() as *const DISK_GEOMETRY_EX) };
                Some(geometry.DiskSize as u64)
            } else {
                None
            };

            unsafe {
                let _ = CloseHandle(h);
            }

            // مقایسه DeviceID با contains روی lowercase
            let norm_path = path.to_ascii_lowercase();
            let extra = wmi_results.iter().find(|d| {
                d.get("DeviceID")
                    .and_then(|v| {
                        if let Variant::String(s) = v {
                            Some(s.to_ascii_lowercase().contains(&norm_path))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false)
            });

            let model = extra
                .and_then(|d| d.get("Model"))
                .and_then(|v| match v {
                    Variant::String(s) => Some(s.clone()),
                    _ => None,
                });

            let serial = extra
                .and_then(|d| d.get("SerialNumber"))
                .and_then(|v| match v {
                    Variant::String(s) => Some(s.clone()),
                    _ => None,
                });

            let is_removable = extra
                .and_then(|d| d.get("MediaType"))
                .map(|v| match v {
                    Variant::String(s) => s.to_lowercase().contains("removable"),
                    _ => false,
                })
                .unwrap_or(false);

            disks.push(DiskInfo {
                name: path,
                size_gb: size.unwrap_or(0) / 1_000_000_000,
                is_removable,
                model,
                serial,
            });
        }
    }

    Ok(disks)
}
