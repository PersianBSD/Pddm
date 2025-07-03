// // مسیر: cli/src/tui.rs

// use ratatui::prelude::*;
// use ratatui::backend::CrosstermBackend;
// use ratatui::widgets::{Block, Borders, Paragraph};
// use std::io;
// use crossterm::{terminal, ExecutableCommand};

// /// تابع شروع رابط کاربری متنی (TUI)
// pub fn start_ui() -> Result<(), io::Error> {
//     // آماده‌سازی ترمینال
//     let mut stdout = io::stdout();
//     terminal::enable_raw_mode()?;
//     stdout.execute(terminal::EnterAlternateScreen)?;

//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     // رسم رابط اولیه
//     terminal.draw(|f| {
//         let size = f.size();
//         let block = Block::default()
//             .title("📦 Pddm - مدیریت دیسک")
//             .borders(Borders::ALL);

//         let paragraph = Paragraph::new("در حال آماده‌سازی رابط...").block(block);
//         f.render_widget(paragraph, size);
//     })?;

//     // بستن رابط
//     terminal::disable_raw_mode()?;
//     io::stdout().execute(terminal::LeaveAlternateScreen)?;

//     Ok(())
// }


pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("✅ رابط ترمینال شروع شد!");
    // ادامه رابط گرافیکی در آینده...
    Ok(())
}