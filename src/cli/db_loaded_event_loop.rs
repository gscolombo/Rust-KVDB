use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::{App, CurrentScreen, DatabaseCommands::*, DatabasePrompt, MainMenu};

use crate::cli::menus::database_commands;
use crate::cli::shared::user_input;

fn _search(app: &mut App) {
    // Call B-Tree search function
}

fn _insert(app: &mut App) {
    let path = std::path::Path::new(&app.input);
    if path
        .try_exists()
        .expect("Não foi possível verificar a existência do arquivo.")
    {
        let file_content = std::fs::read_to_string(path).expect("Não foi possível ler o arquivo.");
        if let Some(prefix) = path.file_prefix() {
            let key = prefix
                .to_str()
                .expect("Não foi possível converter o nome do arquivo para o padrão Unicode.")
                .to_string();

            // Call B-Tree insert function
        }
    }
}

pub fn database_loaded_event_loop(key: KeyEvent, app: &mut App) {
    match app.current_screen {
        CurrentScreen::DatabaseLoaded(DatabasePrompt::SelectCommand) => {
            app.db_command = database_commands(key, app);
            match app.db_command {
                None => {}
                Some(CLOSE) => {
                    app.loaded_db = None;
                    app.current_screen = CurrentScreen::Main(MainMenu::OptionsList);
                    app.option_highlighted = 0;
                    // Remove database index
                }
                _ => {
                    app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::UserInput);
                }
            }
        }
        CurrentScreen::DatabaseLoaded(DatabasePrompt::UserInput)
            if key.kind == KeyEventKind::Press =>
        {
            user_input(
                key,
                app,
                CurrentScreen::DatabaseLoaded(DatabasePrompt::SelectCommand),
                |app| match app.db_command {
                    Some(SEARCH) => _search(app),
                    Some(INSERT) => _insert(app),
                    _ => {}
                },
            );
        }
        CurrentScreen::DatabaseLoaded(DatabasePrompt::SuccessMessage) => match key.code {
            KeyCode::Enter | KeyCode::Esc => {
                app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::SelectCommand);
            }
            _ => {}
        },
        _ => {}
    }
}
