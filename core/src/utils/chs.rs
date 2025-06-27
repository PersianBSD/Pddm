#[derive(Debug, Clone, PartialEq)]
pub struct CHS {
    cylinder: u32,
    head: u32,
    sector: u32,
    spt: u32, // Sectors Per Track
    tpc: u32, // Tracks Per Cylinder
}

impl CHS {
    pub fn new() -> Self {
        CHS {
            cylinder: 0,
            head: 0,
            sector: 0,
            spt: 63,
            tpc: 255,
        }
    }

    pub fn from_chs(c: u32, h: u32, s: u32) -> Self {
        CHS {
            cylinder: c,
            head: h,
            sector: s,
            spt: 63,
            tpc: 255,
        }
    }

    pub fn from_lba(lba: u64) -> Self {
        let mut chs = CHS::new();
        chs.calc_from_lba(lba);
        chs
    }

    pub fn calc_from_lba(&mut self, lba: u64) {
        self.cylinder = (lba / (self.tpc * self.spt) as u64) as u32;
        let temp = (lba % (self.tpc * self.spt) as u64) as u32;
        self.head = temp / self.spt;
        self.sector = temp % self.spt + 1;
    }

    pub fn to_lba(&self) -> u64 {
        ((self.cylinder as u64 * self.tpc as u64 + self.head as u64) * self.spt as u64)
            + self.sector as u64
            - 1
    }

    pub fn set_geometry(&mut self, spt: u32, tpc: u32) {
        let old_lba = self.to_lba();
        self.spt = spt;
        self.tpc = tpc;
        self.calc_from_lba(old_lba);
    }

    pub fn set_spt(&mut self, spt: u32) {
        self.set_geometry(spt, self.tpc);
    }

    pub fn set_tpc(&mut self, tpc: u32) {
        self.set_geometry(self.spt, tpc);
    }

    // Getters
    pub fn cylinder(&self) -> u32 { self.cylinder }
    pub fn head(&self) -> u32 { self.head }
    pub fn sector(&self) -> u32 { self.sector }
    pub fn spt(&self) -> u32 { self.spt }
    pub fn tpc(&self) -> u32 { self.tpc }
}
