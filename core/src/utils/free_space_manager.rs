use crate::utils::types::{FreeSpace, FreeSpaceType, GenPart};

pub struct FreeSpaceManager {
    pub free_spaces: Vec<FreeSpace>,
}

impl FreeSpaceManager {
    pub fn new() -> Self {
        FreeSpaceManager {
            free_spaces: Vec::new(),
        }
    }

    pub fn find_free_spaces(
        &mut self,
        partitions: &[GenPart],
        total_sectors: u64,
        _sector_size: u64,
        _sectors_per_track: u64,
    ) {
        self.find_outside_extended(partitions, total_sectors);
        self.find_inside_extended(partitions);
        self.find_gpt_free(partitions, total_sectors);
        self.sort();
    }

    fn find_outside_extended(&mut self, partitions: &[GenPart], total_sectors: u64) {
        let mut last_end = 0;
        for part in partitions {
            if part.is_logical() {
                continue;
            }
            if part.start_sector > last_end {
                self.free_spaces.push(FreeSpace {
                    begin_sector: last_end,
                    length: part.start_sector - last_end,
                    space_type: FreeSpaceType::Unallocated,
                });
            }
            last_end = part.start_sector + part.total_sectors;
        }

        if last_end < total_sectors {
            self.free_spaces.push(FreeSpace {
                begin_sector: last_end,
                length: total_sectors - last_end,
                space_type: FreeSpaceType::Unallocated,
            });
        }
    }

    fn find_inside_extended(&mut self, partitions: &[GenPart]) {
        let extended = partitions.iter().find(|p| p.is_extended());
        if extended.is_none() {
            return;
        }
        let extended = extended.unwrap();
        let mut logicals: Vec<_> = partitions.iter().filter(|p| p.is_logical()).collect();
        logicals.sort_by_key(|p| p.start_sector);

        let mut last_end = extended.start_sector;
        for part in logicals {
            if part.start_sector > last_end {
                self.free_spaces.push(FreeSpace {
                    begin_sector: last_end,
                    length: part.start_sector - last_end,
                    space_type: FreeSpaceType::Extended,
                });
            }
            last_end = part.start_sector + part.total_sectors;
        }

        let ext_end = extended.start_sector + extended.total_sectors;
        if last_end < ext_end {
            self.free_spaces.push(FreeSpace {
                begin_sector: last_end,
                length: ext_end - last_end,
                space_type: FreeSpaceType::Extended,
            });
        }
    }

    fn find_gpt_free(&mut self, partitions: &[GenPart], total_sectors: u64) {
        let gpt_parts: Vec<_> = partitions.iter().filter(|p| p.is_gpt()).collect();
        if gpt_parts.is_empty() {
            return;
        }

        let mut last_end = 34;
        for part in gpt_parts {
            if part.start_sector > last_end {
                self.free_spaces.push(FreeSpace {
                    begin_sector: last_end,
                    length: part.start_sector - last_end,
                    space_type: FreeSpaceType::Gpt,
                });
            }
            last_end = part.start_sector + part.total_sectors;
        }

        if last_end < total_sectors - 33 {
            self.free_spaces.push(FreeSpace {
                begin_sector: last_end,
                length: total_sectors - 33 - last_end,
                space_type: FreeSpaceType::Gpt,
            });
        }
    }

    fn sort(&mut self) {
        self.free_spaces.sort_by_key(|fs| fs.begin_sector);
    }

    pub fn count(&self) -> usize {
        self.free_spaces.len()
    }

    pub fn get(&self, index: usize) -> Option<&FreeSpace> {
        self.free_spaces.get(index)
    }
}
