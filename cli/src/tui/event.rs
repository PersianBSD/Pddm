// File: cli/src/tui/event.rs


use crate::tui::state::{AppState, Focus};
use crossterm::event::{KeyCode, MouseEvent, MouseEventKind};
use pddm_core::partition;

pub fn handle_event(code: KeyCode, app: &mut AppState) {
    match code {
        KeyCode::Tab => app.toggle_focus(),
        KeyCode::Down => match app.focus {
            Focus::Disk => app.move_disk_selection(1),
            Focus::Partition => app.move_partition_selection(1),
        },
        KeyCode::Up => match app.focus {
            Focus::Disk => app.move_disk_selection(-1),
            Focus::Partition => app.move_partition_selection(-1),
        },
        _ => {}
    }
}

pub fn handle_mouse(event: MouseEvent, app: &mut AppState, layout_height: u16) {
    match event.kind {
        MouseEventKind::Down(_) => {
            let y = event.row.saturating_sub(1); // offset from margin

            if y < layout_height / 3 {
                // Click inside Disks
                let index = y.saturating_sub(2) as usize;
                if index < app.disks.len() {
                    app.disk_state.select(Some(index));
                    if let Some(disk) = app.disks.get(index) {
                        app.partitions = partition::os::windows::get_partitions_for_disk(&disk.disk_name).unwrap_or_default();
                        app.partition_state.select(Some(0));
                    }
                    app.focus = Focus::Disk;
                }
            } else if y < layout_height * 2 / 3 {
                // Click inside Partitions
                let part_index = (y - layout_height / 3 - 2) as usize;
                if part_index < app.partitions.len() {
                    app.partition_state.select(Some(part_index));
                    app.focus = Focus::Partition;
                }
            }
        }
        _ => {}
    }
}
