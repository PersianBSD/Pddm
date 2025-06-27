use std::{
    ffi::OsStr,
   // io::{Read, Seek, SeekFrom, Write},
    os::windows::ffi::OsStrExt,
};

use windows::{
    core::PCWSTR,
    Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    Win32::Storage::FileSystem::{
        CreateFileW, ReadFile, SetFilePointerEx, FILE_GENERIC_READ , FILE_ATTRIBUTE_NORMAL,
         FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,FILE_BEGIN
    },
};

pub struct WindowsDisk {
    handle: HANDLE,
}

impl WindowsDisk {
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let wide: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();

        let handle = unsafe {
            CreateFileW(
                PCWSTR(wide.as_ptr()),
                FILE_GENERIC_READ.0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        };

        let handle = match handle {
            Ok(h) => {
                if h == INVALID_HANDLE_VALUE {
                    return Err("❌ Invalid handle".into());
                }
                h
            }
            Err(e) => return Err(format!("❌ Failed to open handle: {}", e).into()),
        };

        Ok(Self { handle })
    }

    pub fn read_sector(
        &mut self,
        sector_index: u64,
        sector_size: usize,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let offset = sector_index * sector_size as u64;
        let mut new_pos = 0i64;

        let result = unsafe {
          SetFilePointerEx(self.handle, offset as i64, Some(&mut new_pos), FILE_BEGIN,) 
        };

    if let Err(e) = result {
          return Err(format!("❌ Seek failed: {}", e).into());
    }


        let mut buffer = vec![0u8; sector_size];
        let mut bytes_read = 0;

         let success = unsafe {
                ReadFile(
                self.handle,
                Some(&mut buffer),
                //buffer.len() as u32,
                Some(&mut bytes_read),
                None,
            )
        };
    if let Err(e) = success {
        eprintln!("❌ Read failed: {}", e);
        return Err("❌ Read failed".into());
    }


        Ok(buffer)
    }
}

impl Drop for WindowsDisk {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}
