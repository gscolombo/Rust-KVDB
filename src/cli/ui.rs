use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::app::{App, CurrentScreen, MainMenu};

mod button;
mod home;
mod db_listing;

use home::home;
use db_listing::database_listing;

pub fn ui(frame: &mut Frame, app: &mut App) {
    frame.render_widget(Clear, frame.area());

    // Set frame base layout
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Max(3)])
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
        CurrentScreen::Main(MainMenu::OptionsList)
        | CurrentScreen::Main(MainMenu::CreateDb)
        | CurrentScreen::Main(MainMenu::SuccessMessage) => home(frame, app, area),
        CurrentScreen::DatabaseList => database_listing(frame, app, area),
        _ => {}
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Length(height),
            Constraint::Percentage(25),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
