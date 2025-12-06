use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::{App, CurrentScreen};
use crate::cli::ui::db_prompt::database_prompt;

mod button;
mod home;
mod db_listing;
mod db_prompt;
mod shared;

use home::home;
use db_listing::database_listing;

pub fn ui(frame: &mut Frame, app: &mut App) {
    frame.render_widget(Clear, frame.area());

    // Set frame base layout
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(frame.area());

    // Define title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Banco de dados chave-valor",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    // Render title
    frame.render_widget(title, area[0]);

    // Render screen
    match app.current_screen {
        CurrentScreen::Main(_) => home(frame, app, area),
        CurrentScreen::DatabaseList => database_listing(frame, app, area),
        CurrentScreen::DatabaseLoaded(_) => database_prompt(frame, app, area)
    }
}


