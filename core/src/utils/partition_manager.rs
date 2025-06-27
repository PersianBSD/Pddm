use crate::utils::types::{GenPart, PartFlag, DiskError};
use std::collections::HashMap;

#[derive(Default)]
pub struct PartitionManager {
    partitions: Vec<(GenPart, u32)>, // پارتیشن + UID
    least_uid: u32,
}

impl PartitionManager {
    pub fn new() -> Self {
        Self {
            partitions: Vec::new(),
            least_uid: 1,
        }
    }

    pub fn count(&self) -> usize {
        self.partitions.len()
    }

    pub fn add_partition(&mut self, new_part: GenPart) -> Result<u32, DiskError> {
        use PartFlag::*;
        // بررسی محدودیت‌های MBR
        let mbr_limit = self.count_type(Primary | Extended | MbrGpt);
        if new_part.flags.contains(Primary) && mbr_limit >= 4 {
            return Err(DiskError::MbrLimitExceeded);
        }
        if new_part.flags.contains(Extended) {
            if self.count_type(Extended) > 0 {
                return Err(DiskError::OnlyOneExtendedAllowed);
            }
            if mbr_limit >= 4 {
                return Err(DiskError::MbrLimitExceeded);
            }
        }
        if new_part.flags.contains(Logical) {
            if self.count_type(Extended) != 1 {
                return Err(DiskError::ExtendedNotFound);
            }
        }
        if new_part.flags.contains(MbrGpt) {
            if mbr_limit >= 4 || self.count_type(MbrGpt) > 0 {
                return Err(DiskError::GptConflict);
            }
        }

        self.least_uid += 1;
        self.partitions.push((new_part, self.least_uid));
        self.partitions.sort_by_key(|(p, _)| p.begin_sector);
        Ok(self.least_uid)
    }

    pub fn get_partition(&self, index: usize) -> Result<&GenPart, DiskError> {
        self.partitions
            .get(index)
            .map(|(p, _)| p)
            .ok_or(DiskError::OutOfBounds)
    }

    pub fn get_partition_by_uid(&self, uid: u32) -> Result<&GenPart, DiskError> {
        let idx = self.which_by_uid(uid)?;
        self.get_partition(idx)
    }

    pub fn count_type(&self, flag: PartFlag) -> usize {
        self.partitions
            .iter()
            .filter(|(p, _)| p.flags.intersects(flag))
            .count()
    }

    pub fn delete_partition(&mut self, index: usize) -> Result<(), DiskError> {
        let target = self.get_partition(index)?;
        use PartFlag::*;
        if target.flags.contains(Extended) {
            self.partitions
                .retain(|(p, _)| !p.flags.contains(Logical));
        } else if target.flags.contains(MbrGpt) {
            self.partitions
                .retain(|(p, _)| !p.flags.contains(Gpt));
        }

        self.partitions.remove(index);
        Ok(())
    }

    pub fn set_active(&mut self, index: usize, active: bool) -> Result<(), DiskError> {
        use PartFlag::*;
        let part = self.get_partition(index)?;
        if !(part.flags.contains(Primary) || part.flags.contains(MbrGpt)) {
            return Err(DiskError::OnlyPrimaryOrGptCanBeActive);
        }

        for (p, _) in self.partitions.iter_mut() {
            p.flags.remove(Active);
        }

        if active {
            self.partitions[index].0.flags.insert(Active);
        }

        Ok(())
    }

    pub fn which_by_uid(&self, uid: u32) -> Result<usize, DiskError> {
        self.partitions
            .iter()
            .position(|(_, id)| *id == uid)
            .ok_or(DiskError::PartitionNotFound)
    }
}
