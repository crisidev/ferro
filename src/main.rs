use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders},
};
use std::io;
use tokio;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Set up terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create layout
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // Top buffer for channel info
                    Constraint::Min(0),    // Center buffer for chat
                    Constraint::Length(3), // Bottom buffer for typing response
                ]
                .as_ref(),
            )
            .split(f.size());

        let top_block = Block::default()
            .title("Channel Info")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::White));
        f.render_widget(top_block, chunks[0]);

        let center_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(15), // Left buffer for general channel info
                    Constraint::Min(0),     // Center buffer for chat
                    Constraint::Length(15), // Right buffer for users in a channel
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let left_block = Block::default()
            .title("General Info")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::White));
        f.render_widget(left_block, center_chunks[0]);

        let chat_block = Block::default()
            .title("Chat")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::White));
        f.render_widget(chat_block, center_chunks[1]);

        let right_block = Block::default()
            .title("Users")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::White));
        f.render_widget(right_block, center_chunks[2]);

        let bottom_block = Block::default()
            .title("Type here")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::White));
        f.render_widget(bottom_block, chunks[2]);
    })?;

    Ok(())
}
