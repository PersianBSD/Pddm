// مسیر: core/src/util/types/guid.rs

//! ساختارهای مربوط به GUID و شناسه‌های پارتیشن

use std::fmt;
use std::str::FromStr;

/// ساختار یک GUID پایه (نسخه ساده‌شده)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        )
    }
}

impl FromStr for Guid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 5 {
            return Err("Invalid GUID format".into());
        }

        let data1 = u32::from_str_radix(parts[0], 16).map_err(|_| "Invalid data1")?;
        let data2 = u16::from_str_radix(parts[1], 16).map_err(|_| "Invalid data2")?;
        let data3 = u16::from_str_radix(parts[2], 16).map_err(|_| "Invalid data3")?;

        let part4 = parts[3];
        let part5 = parts[4];
        if part4.len() != 4 || part5.len() != 12 {
            return Err("Invalid data4 length".into());
        }

        let mut data4 = [0u8; 8];
        for i in 0..2 {
            data4[i] = u8::from_str_radix(&part4[i * 2..i * 2 + 2], 16).map_err(|_| "Invalid data4")?;
        }
        for i in 0..6 {
            data4[i + 2] = u8::from_str_radix(&part5[i * 2..i * 2 + 2], 16).map_err(|_| "Invalid data4")?;
        }

        Ok(Guid {
            data1,
            data2,
            data3,
            data4,
        })
    }
}
