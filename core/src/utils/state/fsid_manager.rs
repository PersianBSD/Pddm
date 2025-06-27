//! مدیریت شناسه فایل‌سیستم‌های MBR و سازگاری آن‌ها

#[derive(Clone, Debug)]
pub struct FsidInfo {
    pub fsid: u8,
    pub description: String,
    pub min_size: Option<u64>, // به بایت
    pub max_size: Option<u64>,
    pub partmod_fsid: u32,     // شناسه داخلی ابزار
}

pub struct FsidManager {
    fsids: Vec<FsidInfo>,
}

impl FsidManager {
    pub fn new() -> Self {
        let mut mgr = FsidManager { fsids: Vec::new() };
        mgr.add_defaults();
        mgr
    }

    fn add_defaults(&mut self) {
        self.fsids.push(FsidInfo {
            fsid: 0x07,
            description: "NTFS/exFAT/HPFS".into(),
            min_size: Some(512 * 1024 * 1024), // 512MB
            max_size: None,
            partmod_fsid: 0x01,
        });

        self.fsids.push(FsidInfo {
            fsid: 0x0B,
            description: "FAT32 (CHS)".into(),
            min_size: Some(32 * 1024 * 1024), // 32MB
            max_size: Some(2 * 1024 * 1024 * 1024),
            partmod_fsid: 0x02,
        });

        // ادامه موارد دیگر در صورت نیاز اضافه شود
    }

    pub fn get_by_fsid(&self, fsid: u8) -> Option<&FsidInfo> {
        self.fsids.iter().find(|info| info.fsid == fsid)
    }

    pub fn all(&self) -> &[FsidInfo] {
        &self.fsids
    }
}
