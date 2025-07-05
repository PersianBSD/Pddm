// File: cli/src/tui/layout.rs

use ratatui::{
    prelude::Frame,
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Row, Table, Paragraph},
    style::{Style, Color, Modifier},
};
use crate::tui::state::AppState;

pub fn draw_main_ui(f: &mut Frame, app: &mut AppState) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(35),
            Constraint::Percentage(30),
        ])
        .split(area);

    // Disk Table
    let disk_rows: Vec<Row> = app.disks.iter().enumerate().map(|(i, d)| {
        Row::new(vec![
            (i + 1).to_string(),
            d.disk_name.clone(),
            d.model.as_deref().unwrap_or("-").to_string(),
            format!("{:.1} GB", d.size_gb),
        ])
    }).collect();

    let disk_table = Table::new(disk_rows, [
        Constraint::Length(3),
        Constraint::Length(25),
        Constraint::Length(20),
        Constraint::Length(12),
    ])
    .header(Row::new(vec!["#", "Device", "Model", "Size"]).style(Style::default().fg(Color::Yellow)))
    .block(Block::default().borders(Borders::ALL).title("Disks"))
    .row_highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(disk_table, chunks[0], &mut app.disk_state);

    // Partition Table
    let part_rows: Vec<Row> = app.partitions.iter().map(|p| {
        Row::new(vec![
            p.partition_name.clone(),
            p.file_system.as_deref().unwrap_or("-").to_string(),
            p.mount_point.as_deref().unwrap_or("-").to_string(),
            p.volume_label.as_deref().unwrap_or("-").to_string(),
            p.total_space.map_or("-".to_string(), |s| format!("{} GB", s / 1_073_741_824)),
        ])
    }).collect();

    let part_table = Table::new(part_rows, [
        Constraint::Length(12),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(15),
        Constraint::Length(12),
    ])
    .header(Row::new(vec!["Name", "FS", "Mount", "Label", "Size"]).style(Style::default().fg(Color::Yellow)))
    .block(Block::default().borders(Borders::ALL).title("Partitions"))
    .row_highlight_style(Style::default().bg(Color::Magenta).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(part_table, chunks[1], &mut app.partition_state);

    // Partition Detail
    let details = if let Some(sel) = app.partition_state.selected() {
        if let Some(p) = app.partitions.get(sel) {
            vec![
                format!("Name: {}", p.partition_name),
                format!("FS: {}", p.file_system.as_deref().unwrap_or("-")),
                format!("Label: {}", p.volume_label.as_deref().unwrap_or("-")),
                format!("Mount: {}", p.mount_point.as_deref().unwrap_or("-")),
                format!("Size: {}", p.total_space.map_or("-".to_string(), |s| format!("{} GB", s / 1_073_741_824))),
                format!("Boot: {:?}", p.is_boot),
                format!("System: {:?}", p.is_system),
                format!("Hidden: {:?}", p.is_hidden),
                format!("GUID: {}", p.guid_type.as_deref().unwrap_or("-")),
            ].join("\n")
        } else {
            "No partition selected.".to_string()
        }
    } else {
        "No partition selected.".to_string()
    };

    let detail_block = Paragraph::new(details)
        .block(Block::default().borders(Borders::ALL).title("Partition Details"));

    f.render_widget(detail_block, chunks[2]);
}
