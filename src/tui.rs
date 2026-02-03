use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, size},
};
use std::io::{Write, stdout};

pub struct TUI;

impl TUI {
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        Ok(())
    }

    pub fn cleanup() -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), LeaveAlternateScreen, Show)?;
        Ok(())
    }

    pub fn clear() -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn move_to(x: u16, y: u16) -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn print_at(
        x: u16,
        y: u16,
        text: &str,
        color: Color,
    ) -> Result<(), Box<dyn std::error::Error>> {
        execute!(
            stdout(),
            MoveTo(x, y),
            SetForegroundColor(color),
            Print(text),
            ResetColor
        )?;
        Ok(())
    }

    pub fn get_size() -> Result<(u16, u16), Box<dyn std::error::Error>> {
        Ok(size()?)
    }

    pub fn draw_box(
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        color: Color,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Top border
        Self::print_at(x, y, &"─".repeat(width as usize), color)?;

        // Sides
        for i in 1..height {
            Self::print_at(x, y + i, "│", color)?;
            Self::print_at(x + width - 1, y + i, "│", color)?;
        }

        // Bottom border
        Self::print_at(x, y + height, &"─".repeat(width as usize), color)?;

        // Corners
        Self::print_at(x, y, "┌", color)?;
        Self::print_at(x + width - 1, y, "┐", color)?;
        Self::print_at(x, y + height, "└", color)?;
        Self::print_at(x + width - 1, y + height, "┘", color)?;

        Ok(())
    }

    pub fn draw_header(title: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (width, _) = Self::get_size()?;

        Self::print_at(0, 0, &"═".repeat(width as usize), Color::Cyan)?;
        Self::print_at(2, 0, title, Color::Yellow)?;
        Self::print_at(0, 1, &"═".repeat(width as usize), Color::Cyan)?;

        Ok(())
    }

    pub fn draw_status_line(text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (width, height) = Self::get_size()?;

        Self::print_at(
            0,
            height.saturating_sub(1),
            &" ".repeat(width as usize),
            Color::Reset,
        )?;
        Self::print_at(0, height.saturating_sub(1), text, Color::Green)?;

        Ok(())
    }
}
