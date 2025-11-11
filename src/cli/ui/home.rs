use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use std::rc::Rc;

use super::button::Button;
use super::centered_rect;

use crate::App;
use crate::app::CurrentScreen;
use crate::app::MainMenu;

fn render_success_message(frame: &mut Frame) {
    let popup = centered_rect(30, 4, frame.area());

    let message_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::LightGreen))
        .padding(Padding::uniform(1));

    let message = Paragraph::new(
        Text::from("Novo banco de dados criado com sucesso!\nAperte ESC ou ENTER para voltar.")
            .style(Style::default().bg(Color::LightGreen).fg(Color::Black)),
    )
    .centered()
    .block(message_block);

    frame.render_widget(message, popup);
}

fn render_create_db_popup(frame: &mut Frame, app: &mut App) {
    let popup_block = Block::default()
        .title("Insira o nome do banco de dados")
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

pub fn home(frame: &mut Frame, app: &mut App, area: Rc<[Rect]>) {
    // Define layout
    let options_list = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
        .split(area[1]);

    // Initialize options button list
    let mut options = vec![
        Button::new("Criar banco de dados".to_string(), false),
        Button::new("Carregar banco de dados".to_string(), false),
        Button::new("Sair".to_string(), false),
    ];

    // Check if popup is activated
    match app.current_screen {
        CurrentScreen::Main(MainMenu::CreateDb) => {
            render_create_db_popup(frame, app);
        }
        CurrentScreen::Main(MainMenu::SuccessMessage) => {
            render_success_message(frame);
        }
        _ => {}
    }

    // Render menu
    for (_i, opt) in options.iter_mut().enumerate() {
        if _i == app.option_highlighted.into() {
            opt.toggle_highlight();
        }

        let option_p = opt.get_paragraph();
        frame.render_widget(option_p, options_list[_i]);
    }
}
