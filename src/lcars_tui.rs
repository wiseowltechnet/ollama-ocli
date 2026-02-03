use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

pub struct LcarsTUI {
    pub show_dashboard: bool,
}

impl LcarsTUI {
    pub fn new() -> Self {
        Self { show_dashboard: false }
    }

    pub fn render(
        &self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        stats: &crate::dashboard::DashboardStats,
        model: &str,
        session: &str,
        mcp_count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),  // Header
                    Constraint::Min(10),    // Main area
                    Constraint::Length(3),  // Footer
                ])
                .split(f.size());

            self.render_header(f, chunks[0], model, session);
            
            if self.show_dashboard {
                self.render_dashboard(f, chunks[1], stats, mcp_count);
            } else {
                self.render_compact_stats(f, chunks[1], stats);
            }
            
            self.render_footer(f, chunks[2]);
        })?;
        Ok(())
    }

    fn render_header(&self, f: &mut Frame, area: Rect, model: &str, session: &str) {
        let header = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Rgb(255, 153, 0)))
            .title(format!(" ▐█▌ LCARS OLLAMA CLI "));
        
        let text = vec![
            Line::from(vec![
                Span::styled("Model: ", Style::default().fg(Color::Rgb(153, 204, 255))),
                Span::raw(model),
                Span::raw("  "),
                Span::styled("Session: ", Style::default().fg(Color::Rgb(204, 153, 255))),
                Span::raw(session),
            ]),
        ];
        
        let paragraph = Paragraph::new(text).block(header);
        f.render_widget(paragraph, area);
    }

    fn render_compact_stats(&self, f: &mut Frame, area: Rect, stats: &crate::dashboard::DashboardStats) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(area);

        // Memory gauge
        let mem_mb = stats.memory_usage / 1024 / 1024;
        let mem_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(" Memory "))
            .gauge_style(Style::default().fg(Color::Rgb(255, 153, 0)))
            .ratio((mem_mb as f64 / 100.0).min(1.0))
            .label(format!("{}MB", mem_mb));
        f.render_widget(mem_gauge, chunks[0]);

        // Stats
        let stats_text = vec![
            Line::from(format!("Turns: {}", stats.turn_count)),
            Line::from(format!("Tokens: {}", stats.token_count)),
            Line::from(format!("Avg Response: {:.1}s", stats.avg_response_time())),
            Line::from(format!("Uptime: {}s", stats.uptime().as_secs())),
        ];
        let stats_widget = Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).title(" Stats "));
        f.render_widget(stats_widget, chunks[1]);

        // Activity
        let items: Vec<ListItem> = stats.activity_log.iter()
            .map(|s| ListItem::new(s.as_str()))
            .collect();
        let activity = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Activity "));
        f.render_widget(activity, chunks[2]);
    }

    fn render_dashboard(&self, f: &mut Frame, area: Rect, stats: &crate::dashboard::DashboardStats, mcp_count: usize) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(area);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(chunks[0]);

        // Response times
        let response_bars: Vec<String> = stats.response_times.iter()
            .map(|t| {
                let bars = (t * 3.0) as usize;
                format!("{} {:.1}s", "█".repeat(bars.min(20)), t)
            })
            .collect();
        let response_text: Vec<Line> = response_bars.iter()
            .map(|s| Line::from(s.as_str()))
            .collect();
        let response_widget = Paragraph::new(response_text)
            .block(Block::default().borders(Borders::ALL).title(" Response Times "))
            .style(Style::default().fg(Color::Rgb(255, 153, 0)));
        f.render_widget(response_widget, top_chunks[0]);

        // Memory
        let mem_mb = stats.memory_usage / 1024 / 1024;
        let mem_text = vec![
            Line::from(format!("OCLI: {}MB", mem_mb)),
            Line::from(format!("Turns: {}", stats.turn_count)),
            Line::from(format!("Tokens: {}", stats.token_count)),
        ];
        let mem_widget = Paragraph::new(mem_text)
            .block(Block::default().borders(Borders::ALL).title(" Memory "));
        f.render_widget(mem_widget, top_chunks[1]);

        // MCP
        let mcp_text = vec![
            Line::from(format!("✓ {} tools loaded", mcp_count)),
        ];
        let mcp_widget = Paragraph::new(mcp_text)
            .block(Block::default().borders(Borders::ALL).title(" MCP Tools "));
        f.render_widget(mcp_widget, top_chunks[2]);

        // Activity log
        let items: Vec<ListItem> = stats.activity_log.iter()
            .map(|s| ListItem::new(s.as_str()))
            .collect();
        let activity = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Activity Log "));
        f.render_widget(activity, chunks[1]);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let footer = Paragraph::new("[F3] Dashboard  [Ctrl+C] Exit  [↑↓] History")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Rgb(153, 204, 255)));
        f.render_widget(footer, area);
    }
}
