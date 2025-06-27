use crate::util::types::{GenPart, PartitionFlag};
use crate::util::disk::Disk;
use crate::util::mbr::{Mbr, PartitionRecord, MbrSpecific, BS_MAGIC};
use crate::util::fsid_manager::FsidManager;
use crate::util::defs::{FS_EXTENDED, FS_GPT, FSID_EXTENDED, FSID_GPT};
use crate::util::chs::CHS;
use crate::util::exception::{DiskError, DiskResult};

pub struct LegacyMBR<'a> {
    pub disk: &'a mut Disk,
}

impl<'a> LegacyMBR<'a> {
    pub fn new(disk: &'a mut Disk) -> Self {
        Self { disk }
    }

    pub fn read_partition_table(&mut self) -> DiskResult<()> {
        let mut mbr = Mbr::default();
        self.disk.read_mbr(&mut mbr)?;

        if mbr.signature != BS_MAGIC {
            return Err(DiskError::InvalidMBRSignature);
        }

        for (i, rec) in mbr.partition_table.iter().enumerate() {
            if rec.lba_blocks == 0 {
                continue;
            }

            let mut gpart = GenPart {
                begin_sector: rec.begin_lba as u64,
                total_sectors: rec.lba_blocks as u64,
                flags: PartitionFlag::empty(),
                ..Default::default()
            };

            let fsid = rec.partition_type;
            if gpart.begin_sector + gpart.total_sectors > self.disk.last_sector()? {
                return Err(DiskError::PartitionOutsideDisk);
            }

            let fsid_value = self.disk.fsid_manager().get_or_add(fsid);

            let mspec = MbrSpecific {
                fsid,
                begin_sector_rel: 0,
            };

            gpart.fsid = fsid_value;

            if fsid_value == FS_EXTENDED {
                gpart.flags |= PartitionFlag::Extended;
            } else if fsid_value == FS_GPT {
                gpart.flags |= PartitionFlag::MbrGpt;
            } else {
                gpart.flags |= PartitionFlag::Primary;
            }

            if rec.status == 0x80 {
                gpart.flags |= PartitionFlag::Active;
            }

            self.disk.add_partition(gpart);
            self.disk.set_mbr_specific(i, mspec);
        }

        Ok(())
    }

    pub fn write_partition_table(&mut self) -> DiskResult<()> {
        let mut mbr = Mbr::default();
        self.disk.read_mbr(&mut mbr)?;

        if self.disk.partition_count_with_flags(PartitionFlag::Primary | PartitionFlag::Extended | PartitionFlag::MbrGpt) > 4 {
            return Err(DiskError::MBRLimitExceeded);
        }

        for i in 0..4 {
            mbr.partition_table[i] = PartitionRecord::default();
        }

        let chs = CHS::new(
            self.disk.geometry().sectors_per_track,
            self.disk.geometry().tracks_per_cylinder,
        );

        let mut j = 0;
        for i in 0..self.disk.partition_count() {
            let gpart = self.disk.get_partition(i);
            if gpart.flags.intersects(PartitionFlag::Primary | PartitionFlag::Extended | PartitionFlag::MbrGpt) {
                let mspec = self.disk.get_mbr_specific(i);
                let rec = &mut mbr.partition_table[j];
                rec.begin_lba = gpart.begin_sector as u32;
                rec.lba_blocks = gpart.total_sectors as u32;
                rec.partition_type = if gpart.flags.contains(PartitionFlag::Extended) {
                    FSID_EXTENDED
                } else if gpart.flags.contains(PartitionFlag::MbrGpt) {
                    FSID_GPT
                } else {
                    mspec.fsid
                };
                rec.status = if gpart.flags.contains(PartitionFlag::Active) { 0x80 } else { 0x00 };
                rec.begin_chs = chs.from_lba(gpart.begin_sector);
                rec.end_chs = chs.from_lba(gpart.begin_sector + gpart.total_sectors);
                j += 1;
            }
        }

        if mbr.disk_signature == 0 {
            mbr.disk_signature = rand::random();
        }
        mbr.signature = BS_MAGIC;

        self.disk.write_sector(0, &mbr)?;

        Ok(())
    }
}
