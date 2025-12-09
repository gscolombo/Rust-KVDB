use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::{App, CurrentScreen, DatabaseCommands::*, DatabasePrompt, MainMenu};

use crate::btree::BTree;
use crate::cli::menus::database_commands;
use crate::cli::shared::user_input;

fn _search(app: &mut App) {
    if let Some(pager) = &mut app.loaded_db {
        match app.index.search(&app.input, pager) {
            Some(s) => {
                app.search_result = s;
                app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::ResultView);
            }
            None => {
                app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::FailureMessage);
            }
        }
    }
}

fn _insert(app: &mut App) {
    let path = std::path::Path::new(&app.input);
    if path
        .try_exists()
        .expect("Não foi possível verificar a existência do arquivo.")
    {
        let key = path
            .file_stem()  // Isso é estável e retorna o nome sem extensão
            .expect("Não foi possível extrair o prefixo do arquivo")
            .to_str()
            .expect("Não foi possível converter o nome do arquivo para o padrão Unicode.")
            .to_string();

        match std::fs::read_to_string(path) {
            Ok(data) => {
                if let Some(pager) = &mut app.loaded_db {
                    match app.index.insert(key, data, pager) {
                        Ok(_) => {
                            app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::SuccessMessage);
                        }
                        Err(e) => {
                            panic!("{e}");
                            app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::FailureMessage);
                        }
                    }
                }
             }
            Err(e) => {
                panic!("{e}");
                app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::FailureMessage);
            }
        }
    }
}

fn _delete(app: &mut App) {
    if let Some(pager) = &mut app.loaded_db {
        match app.index.delete(app.input.clone(), pager) {
            Ok(_) => {
                app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::SuccessMessage);
            }
            Err(e) => {
                println!("Erro ao deletar: {:?}", e);
                app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::FailureMessage);
            }
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
                    app.index = BTree::default();
                    app.current_screen = CurrentScreen::Main(MainMenu::OptionsList);
                    app.option_highlighted = 0;
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
                    Some(DELETE) => _delete(app),
                    _ => {}
                },
            );
        }
        CurrentScreen::DatabaseLoaded(DatabasePrompt::SuccessMessage) => match key.code {
            KeyCode::Enter | KeyCode::Esc => {
                app.input.clear();
                app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::SelectCommand);
            }
            _ => {}
        },
        CurrentScreen::DatabaseLoaded(DatabasePrompt::ResultView) => match key.code {
            KeyCode::Down => {
                if (app.vertical_scroll as usize) < app.line_count {
                    app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                }
            }
            KeyCode::Up => {
                if app.vertical_scroll > 0 {
                    app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                }
            }
            KeyCode::Esc => {
                app.search_result.clear();
                app.input.clear();
                app.vertical_scroll = 0;
                app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::SelectCommand);
            }
            _ => {}
        },
        _ => {}
    }
}
