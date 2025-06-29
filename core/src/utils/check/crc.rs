// lib/src/crc.rs

pub struct CRC32 {
    table: [u32; 256],
    value: u32,
}

impl CRC32 {
    pub fn new() -> Self {
        let mut table = [0u32; 256];
        for i in 0..256 {
            let mut crc = i as u32;
            for _ in 0..8 {
                if crc & 1 != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
            table[i] = crc;
        }
        CRC32 { table, value: 0xFFFFFFFF }
    }

    pub fn update(&mut self, data: &[u8]) {
        for &byte in data {
            let index = ((self.value ^ byte as u32) & 0xFF) as usize;
            self.value = (self.value >> 8) ^ self.table[index];
        }
    }

    pub fn finalize(&self) -> u32 {
        !self.value
    }

    pub fn checksum(data: &[u8]) -> u32 {
        let mut crc = CRC32::new();
        crc.update(data);
        crc.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32() {
        let data = b"123456789";
        let result = CRC32::checksum(data);
        assert_eq!(result, 0xCBF43926);
    }
}
