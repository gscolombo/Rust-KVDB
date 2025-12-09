use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols::{scrollbar};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Scrollbar, Wrap};

use crate::App;

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
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
        Text::from(message).style(Style::default().bg(Color::LightGreen).fg(Color::Black)),
    )
    .centered()
    .block(message_block);

    frame.render_widget(message, popup);
}

pub fn render_result_view(frame: &mut Frame, app: &mut App) {
    let popup = centered_rect(50, 20, frame.area());

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .split(popup);

    let message_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::LightGreen))
        .padding(Padding::uniform(1))
        .title(format!(" Arquivo {}", app.input))
        .title_bottom(" Pressione ESC para voltar");

    let text = Text::from(app.search_result.clone())
        .style(Style::default().bg(Color::LightGreen).fg(Color::Black));

    let message = Paragraph::new(text.clone())
        .block(message_block.clone())
        .scroll((app.vertical_scroll, 0))
        .wrap(Wrap { trim: true });

    frame.render_widget(message, layout[0]);

    let scrollbar = Scrollbar::default()
        .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
        .symbols(scrollbar::DOUBLE_VERTICAL)
        .begin_symbol(None)
        .track_symbol(None)
        .end_symbol(None);

    app.line_count = line_count(app.search_result.as_str(), layout[0].width);

    let mut scrollbar_state = app
        .vertical_scroll_state
        .content_length(app.line_count)
        .position(app.vertical_scroll as usize);

    frame.render_stateful_widget(scrollbar, layout[1], &mut scrollbar_state);
}

fn line_count(s: &str, width: u16) -> usize {
    let mut line_count = 1;
    let mut min_line_count = 0;

    if width != 0 {
        min_line_count = s.chars().count().div_ceil(width.into());

        let words: Vec<&str> = s.split_whitespace().collect();
        let mut current_line_length: usize = 0;
        
        for word in words {
            current_line_length += word.len();

            if current_line_length > width.into() {
                line_count += 1;
                current_line_length = word.len();
            }
        }
    }

    std::cmp::max(min_line_count, line_count)
}
