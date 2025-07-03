// مسیر: cli/src/tui/mod.rs

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, List, ListItem, ListState},
    layout::{Layout, Constraint, Direction},
    style::{Style, Modifier, Color},
};

use pddm_core::disk::local::get_disks;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(result?)
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    let disks = get_disks();
    let mut selected = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("🖴 لیست دیسک‌ها - ↑↓ برای حرکت، Enter برای انتخاب، q برای خروج")
                .borders(Borders::ALL);

            let items: Vec<ListItem> = disks
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    let name = &d.disk_name;
                    let model = d.model.clone().unwrap_or("ناشناخته".to_string());
                    let text = format!("Disk {}: {} ({})", i, name, model);
                    ListItem::new(text)
                })
                .collect();

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .highlight_symbol("👉 ");

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0)].as_ref())
                .split(size);

            f.render_stateful_widget(list, chunks[0], &mut make_list_state(selected));
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => {
                        if selected < disks.len().saturating_sub(1) {
                            selected += 1;
                        }
                    }
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        // future: show partitions
                    }
                    _ => {}
                }
            }
        }
    }
}

fn make_list_state(selected: usize) -> ListState {
    let mut state = ListState::default();
    state.select(Some(selected));
    state
}
