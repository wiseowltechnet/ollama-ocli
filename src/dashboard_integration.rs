// Minimal F3 dashboard toggle integration

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::Duration;

pub fn check_f3_toggle() -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(KeyEvent { code: KeyCode::F(3), .. }) = event::read()? {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn render_dashboard_frame(
    stats: &crate::dashboard::DashboardStats,
    model: &str,
    session: &str,
    mcp_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    use ratatui::{backend::CrosstermBackend, Terminal};
    use std::io;
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    terminal.clear()?;
    
    let tui = crate::lcars_tui::LcarsTUI { show_dashboard: true };
    tui.render(&mut terminal, stats, model, session, mcp_count)?;
    
    // Wait for F3 to exit dashboard
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::F(3) | KeyCode::Esc) {
                    break;
                }
            }
        }
    }
    
    disable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}
