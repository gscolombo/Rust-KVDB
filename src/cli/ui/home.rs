use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::rc::Rc;

use super::button::Button;

use crate::App;
use crate::app::MainMenu;
use crate::app::{CurrentScreen, DatabaseCommands, DatabasePrompt};
use crate::cli::ui::shared::{render_user_input_popup, render_success_message};


pub fn home(frame: &mut Frame, app: &mut App, area: Rc<[Rect]>) {
    // Define layout
    let options_list = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
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
            render_user_input_popup(frame, app, "Insira o nome do banco de dados");
        }
        CurrentScreen::DatabaseLoaded(DatabasePrompt::UserInput) => match app.db_command {
            Some(DatabaseCommands::INSERT) => {
                render_user_input_popup(frame, app, "Insira o caminho para o arquivo: ");
            }
            _ => {
                render_user_input_popup(frame, app, "Insira a chave: ");
            }
        },
        CurrentScreen::Main(MainMenu::SuccessMessage) => {
            render_success_message(frame, "Novo banco de dados criado com sucesso!\nAperte ESC ou ENTER para voltar.");
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
