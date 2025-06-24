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

pub fn smart_disks_list() -> Result<Vec<DiskInfo>, Box<dyn std::error::Error>> {
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

            let mut out_buffer = [0u8; std::mem::size_of::<DISK_GEOMETRY_EX>() + 1024];
            let mut returned = 0u32;

            let size = unsafe {
                let res = DeviceIoControl(
                    h,
                    IOCTL_DISK_GET_DRIVE_GEOMETRY_EX,
                    None,
                    0,
                    Some(out_buffer.as_mut_ptr() as *mut _),
                    out_buffer.len() as u32,
                    Some(&mut returned),
                    None,
                );

                if let Ok(_) = res {
                    let geometry = &*(out_buffer.as_ptr() as *const DISK_GEOMETRY_EX);
                    Some(geometry.DiskSize as u64)
                } else {
                    None
                }
            };

            unsafe {
                let _ = CloseHandle(h);
            }

            let norm_path = format!("\\\\.\\physicaldrive{}", i);
            let extra = wmi_results.iter().find(|d| {
                d.get("DeviceID").and_then(|v| {
                    if let Variant::String(s) = v {
                        Some(s.to_ascii_lowercase().contains(&norm_path))
                    } else {
                        None
                    }
                }).unwrap_or(false)
            });

            let model = extra.and_then(|d| extract_string(d, "Model"));
            let serial = extra.and_then(|d| extract_string(d, "SerialNumber"));
            let partition_style = extra.and_then(|d| extract_string(d, "PartitionStyle"));

            disks.push(DiskInfo {
                disk_name: path.clone(),
                size_gb: size.unwrap_or(0),
                is_removable: false, // Already known from WMI but ignored here
                model,
                serial,
                partition_style: partition_style.clone(),
                bus_type: extra.and_then(|d| extract_string(d, "BusType")),
                media_type: extra.and_then(|d| extract_string(d, "MediaType")),         
            });
        }
    }

    Ok(disks)
}

fn extract_string(map: &HashMap<String, Variant>, key: &str) -> Option<String> {
    map.get(key).and_then(|v| {
        if let Variant::String(s) = v {
            Some(s.clone())
        } else {
            None
        }
    })
}
