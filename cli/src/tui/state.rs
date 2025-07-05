// File: cli/src/tui/state.rs

use ratatui::widgets::TableState;
use pddm_core::disk::os::windows::smart_disks_list;
use pddm_core::partition::os::windows::get_partitions_for_disk;
use pddm_core::utils::types::info::PartitionInfo;
use pddm_core::utils::types::info::DiskInfo;

#[derive(PartialEq)]
pub enum Focus {
    Disk,
    Partition,
}

pub struct AppState {
    pub disks: Vec<DiskInfo>,
    pub partitions: Vec<PartitionInfo>,
    pub disk_state: TableState,
    pub partition_state: TableState,
    pub focus: Focus,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let disks = smart_disks_list()?;
        let mut disk_state = TableState::default();
        disk_state.select(Some(0));

        let partitions = if let Some(d) = disks.get(0) {
            get_partitions_for_disk(&d.disk_name)?
        } else {
            vec![]
        };

        let mut partition_state = TableState::default();
        if !partitions.is_empty() {
            partition_state.select(Some(0));
        }

        Ok(Self {
            disks,
            partitions,
            disk_state,
            partition_state,
            focus: Focus::Disk,
        })
    }

    pub fn toggle_focus(&mut self) {
        self.focus = if self.focus == Focus::Disk {
            Focus::Partition
        } else {
            Focus::Disk
        };
    }

    pub fn move_disk_selection(&mut self, offset: isize) {
        let len = self.disks.len();
        let i = match self.disk_state.selected() {
            Some(i) => (i as isize + offset).clamp(0, len.saturating_sub(1) as isize) as usize,
            None => 0,
        };

        self.disk_state.select(Some(i));

        if let Some(disk) = self.disks.get(i) {
            let parts = get_partitions_for_disk(&disk.disk_name).unwrap_or_default();
            self.partitions = parts;
            if !self.partitions.is_empty() && self.partition_state.selected().is_none() {
                self.partition_state.select(Some(0));
            }
        }
    }

    pub fn move_partition_selection(&mut self, offset: isize) {
        let len = self.partitions.len();
        let i = match self.partition_state.selected() {
            Some(i) => (i as isize + offset).clamp(0, len.saturating_sub(1) as isize) as usize,
            None => 0,
        };
        self.partition_state.select(Some(i));
    }
}
