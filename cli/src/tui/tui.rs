// // File: cli/src/tui/tui.rs

// use std::io;
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode,},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use ratatui::{
//     backend::CrosstermBackend,
//     Terminal,
//     layout::{Layout, Constraint, Direction},
//     widgets::{Block, Borders, Row, Table, TableState, Paragraph,},
//     style::{Style, Color, Modifier},
// };


// use pddm_core::disk::os::windows::smart_disks_list;
// use pddm_core::partition::os::windows::get_partitions_for_disk;
// use pddm_core::utils::convert::format_bytes;

// #[derive(PartialEq)]
// enum Focus {
//     Disk,
//     Partition,
// }

// pub fn run_tui_interface() -> Result<(), Box<dyn std::error::Error>> {
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     let res = run_app(&mut terminal);

//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     if let Err(err) = res {
//         println!("Error: {:?}", err);
//     }

//     Ok(())
// }

// fn run_app<B: ratatui::backend::Backend>(
//     terminal: &mut Terminal<B>,
// ) -> io::Result<()> {
//     let disks = match smart_disks_list() {
//         Ok(d) => {
//             if d.is_empty() {
//                 println!("⚠️ No disks found.");
//                 return Ok(());
//             }
//             d
//         }
//         Err(e) => {
//             println!("Failed to load disks: {:?}", e);
//             return Ok(());
//         }
//     };

//     let mut disk_state = TableState::default();
//     disk_state.select(Some(0));

//     let mut partition_state = TableState::default();
//     let mut focus = Focus::Disk;

//     loop {
//         let selected_disk_index = disk_state.selected().unwrap_or(0).min(disks.len().saturating_sub(1));
//         let disk = &disks[selected_disk_index];
//         let partitions = get_partitions_for_disk(&disk.disk_name).unwrap_or_default();

//         terminal.draw(|f| {
//             let size = f.size();
//             let chunks = Layout::default()
//                 .direction(Direction::Horizontal)
//                 .margin(1)
//                 .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
//                 .split(size);

//             // Draw disk table
//             let disk_rows: Vec<Row> = disks
//                 .iter()
//                 .enumerate()
//                 .map(|(i, d)| Row::new(vec![
//                     (i + 1).to_string(),
//                     d.disk_name.clone(),
//                     d.model.as_deref().unwrap_or("-").to_string(),
//                     format!("{:.1} GB", d.size_gb),
//                 ]))
//                 .collect();

//             let disk_table = Table::new(disk_rows, [
//                 Constraint::Length(3),
//                 Constraint::Length(25),
//                 Constraint::Length(20),
//                 Constraint::Length(12),
//             ])
//             .header(Row::new(vec!["#", "Device", "Model", "Size"]).style(Style::default().fg(Color::Yellow)))
//             .block(Block::default().borders(Borders::ALL).title("Disks").style(Style::default().bg(Color::DarkGray)))
//             .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD));

//             f.render_stateful_widget(disk_table, chunks[0], &mut disk_state);

//             let selected_disk_index = disk_state.selected().unwrap_or(0).min(disks.len().saturating_sub(1));
//             let disk = &disks[selected_disk_index];

//             let partitions = get_partitions_for_disk(&disk.disk_name).unwrap_or_default();

//             if partition_state.selected().is_none() && !partitions.is_empty() {
//                 partition_state.select(Some(0));
//             }

//             let part_chunks = Layout::default()
//                 .direction(Direction::Vertical)
//                 .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
//                 .split(chunks[1]);

//             // Draw partition table
//             let part_rows: Vec<Row> = partitions
//                 .iter()
//                 .map(|p| Row::new(vec![
//                     p.partition_name.clone(),
//                     p.file_system.as_deref().unwrap_or("-").to_string(),
//                     p.mount_point.as_deref().unwrap_or("-").to_string(),
//                     p.volume_label.as_deref().unwrap_or("-").to_string(),
//                     p.total_space.map_or("-".to_string(), |s| format_bytes(s)),
//                 ]))
//                 .collect();

//             let part_table = Table::new(part_rows, [
//                 Constraint::Length(12),
//                 Constraint::Length(10),
//                 Constraint::Length(10),
//                 Constraint::Length(15),
//                 Constraint::Length(12),
//             ])
//             .header(Row::new(vec!["Name", "FS", "Mount", "Label", "Size"]).style(Style::default().fg(Color::Yellow)))
//             .block(Block::default().borders(Borders::ALL).title("Partitions").style(Style::default().bg(Color::Black)))
//             .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD));

//             f.render_stateful_widget(part_table, part_chunks[0], &mut partition_state);

//             // Partition details
//             let detail = if let Some(sel) = partition_state.selected() {
//                 if let Some(p) = partitions.get(sel) {
//                     vec![
//                         format!("Partition: {}", p.partition_name),
//                         format!("Mount: {}", p.mount_point.as_deref().unwrap_or("-")),
//                         format!("FS: {}", p.file_system.as_deref().unwrap_or("-")),
//                         format!("Label: {}", p.volume_label.as_deref().unwrap_or("-")),
//                         format!("Size: {}", p.total_space.map_or("-".to_string(), |s| format_bytes(s))),
//                         format!("Offset: {:?}", p.offset),
//                         format!("Boot: {:?}", p.is_boot),
//                         format!("System: {:?}", p.is_system),
//                         format!("Hidden: {:?}", p.is_hidden),
//                         format!("GUID Type: {}", p.guid_type.as_deref().unwrap_or("-")),
//                     ].join("\n")
//                 } else {
//                     "No partition selected.".to_string()
//                 }
//             } else {
//                 "No partition selected.".to_string()
//             };

//             let detail_block = Paragraph::new(detail)
//                 .block(Block::default().borders(Borders::ALL).title("Partition Details"));

//             f.render_widget(detail_block, part_chunks[1]);
//         })?;

//         if event::poll(std::time::Duration::from_millis(10000))? {
//             match event::read()? {
//                 CEvent::Key(key) => match key.code {
//                     KeyCode::Char('q') => break,
//                     KeyCode::Tab => {
//                         focus = if focus == Focus::Disk { Focus::Partition } else { Focus::Disk };
//                     },
//                     KeyCode::Down => {
//                         match focus {
//                             Focus::Disk => move_selection(1, &mut disk_state, disks.len()),
//                             Focus::Partition => move_selection(1, &mut partition_state, partitions.len()),
//                         }
//                     },
//                     KeyCode::Up => {
//                         match focus {
//                             Focus::Disk => move_selection(-1, &mut disk_state, disks.len()),
//                             Focus::Partition => move_selection(-1, &mut partition_state, partitions.len()),
//                         }
//                     },
//                     _ => {}
//                 },
//                 _ => {}
//             }
//         }
//     }

//     Ok(())
// }

// fn move_selection(offset: isize, state: &mut TableState, len: usize) {
//     let i = match state.selected() {
//         Some(i) => i as isize + offset,
//         None => 0,
//     };
//     let i = i.clamp(0, len.saturating_sub(1) as isize) as usize;
//     state.select(Some(i));
// }

// File: cli/src/tui/test_app.rs

// File: cli/src/tui/test_app.rs
// File: cli/src/tui/tui.rs

// File: cli/src/tui/tui.rs

use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},// MouseEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use super::layout::draw_main_ui;
use super::event::{handle_event, handle_mouse};
use super::state::AppState;

pub fn run_tui_interface() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new()?;

    loop {
        terminal.draw(|f| draw_main_ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                CEvent::Key(key) => {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                    handle_event(key.code, &mut app);
                }
                CEvent::Mouse(mouse_event) => {
                    let height = terminal.size()?.height;
                    handle_mouse(mouse_event, &mut app, height);
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
