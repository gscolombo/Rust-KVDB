use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::rc::Rc;

use super::button::Button;

use crate::App;
use crate::app::{CurrentScreen, DatabasePrompt, DatabaseCommands::*};
use crate::cli::ui::shared::{render_success_message, render_user_input_popup};

pub fn database_prompt(frame: &mut Frame, app: &mut App, area: Rc<[Rect]>) {
    let options: Vec<String> = vec![SEARCH, INSERT, DELETE, CLOSE]
        .iter()
        .map(|cmd| cmd.to_string())
        .collect();

    // Define layout
    let options_list = Layout::default()
        .direction(Direction::Vertical)
        .constraints(options.iter().map(|_| Constraint::Length(1)))
        .split(area[1]);

    // Initialize options button list
    let buttons = options
        .iter()
        .map(|name| Button::new(name.clone(), false));

    // Check if popup is activated
    match app.current_screen {
        CurrentScreen::DatabaseLoaded(DatabasePrompt::UserInput) => match app.db_command {
            Some(INSERT) => {
                render_user_input_popup(frame, app, "Insira o caminho para o arquivo: ");
            }
            _ => {
                render_user_input_popup(frame, app, "Insira a chave: ");
            }
        },
        CurrentScreen::DatabaseLoaded(DatabasePrompt::SuccessMessage) => {
            render_success_message(frame, "Operação realizada com sucesso!\nAperte ESC ou ENTER para voltar.");
        }
        _ => {}
    }

    // Render menu
    for (_i, mut btn) in buttons.enumerate() {
        if _i == app.option_highlighted.into() {
            btn.toggle_highlight();
        }

        let option_p = btn.get_paragraph();
        frame.render_widget(option_p, options_list[_i]);
    }
}
