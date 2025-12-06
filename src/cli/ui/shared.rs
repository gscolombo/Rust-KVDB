use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph, Padding};

use crate::App;

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

pub fn render_user_input_popup(frame: &mut Frame, app: &mut App, message: &str) {
    let popup_block = Block::default()
        .title(message)
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let popup_area = centered_rect(60, 5, frame.area());
    frame.render_widget(popup_block, popup_area);

    let popup_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(100)])
        .split(popup_area);

    let input_block = Block::default()
        .title("Nome")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::LightGreen).fg(Color::Black));

    let input = Paragraph::new(app.input.clone()).block(input_block);

    frame.render_widget(input, popup_chunks[0]);
}

pub fn render_success_message(frame: &mut Frame, message: &str) {
    let popup = centered_rect(30, 4, frame.area());

    let message_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::LightGreen))
        .padding(Padding::uniform(1));

    let message = Paragraph::new(
        Text::from(message)
            .style(Style::default().bg(Color::LightGreen).fg(Color::Black)),
    )
    .centered()
    .block(message_block);

    frame.render_widget(message, popup);
}