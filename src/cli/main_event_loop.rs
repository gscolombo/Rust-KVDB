use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::{App, CurrentScreen, MainMenu};
use crate::db::create_database;

use crate::cli::menus::{MainMenuOptions, main_menu};
use crate::cli::shared::user_input;

pub fn main_menu_event_loop(key: KeyEvent, app: &mut App) -> bool {
    match app.current_screen {
        CurrentScreen::Main(MainMenu::OptionsList) => match main_menu(key, app) {
            Some(MainMenuOptions::CreateDb) => {
                app.current_screen = CurrentScreen::Main(MainMenu::CreateDb)
            }
            Some(MainMenuOptions::LoadDb) => {
                app.option_highlighted = 0;
                app.current_screen = CurrentScreen::DatabaseList;
            }
            Some(MainMenuOptions::Exit) => return true,
            None => {}
        },
        CurrentScreen::Main(MainMenu::CreateDb) if key.kind == KeyEventKind::Press => {
            user_input(
                key,
                app,
                CurrentScreen::Main(MainMenu::OptionsList),
                |app| {
                    create_database(&app.input).expect("Banco de dados deveria ter sido criado");
                    app.input.clear();
                    app.current_screen = CurrentScreen::Main(MainMenu::SuccessMessage);
                    app.fetch_databases();
                },
            );
        }
        CurrentScreen::Main(MainMenu::SuccessMessage) => match key.code {
            KeyCode::Enter | KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main(MainMenu::OptionsList);
            }
            _ => {}
        },
        _ => {}
    }

    false
}
