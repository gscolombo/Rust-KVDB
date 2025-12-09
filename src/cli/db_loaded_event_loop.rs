use std::io::{Seek, Write};
use std::os::unix::fs::FileExt;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::{App, CurrentScreen, DatabaseCommands::*, DatabasePrompt, MainMenu};

use crate::btree::BTree;
use crate::cli::menus::database_commands;
use crate::cli::shared::user_input;
use crate::records::{create_record, serialize_record};

fn _search(app: &mut App) {
    if let Some(offset) = app.index.search(&app.input)
        && let Some(db) = &mut app.loaded_db
    {
        let mut value_length = [0u8; 8];

        if let Ok(_) = db.read_exact_at(&mut value_length, offset) {
            let size = u64::from_be_bytes(value_length);

            let mut data = vec![0u8; size as usize];
            db.read_exact_at(&mut data, offset + 8)
                .expect("Erro durante leitura de arquivo do banco de dados");

            app.search_result =
                String::from_utf8(data).expect("Erro durante conversão de bytes em string UTF-8");
            app.current_screen = CurrentScreen::DatabaseLoaded(DatabasePrompt::ResultView);
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
            .file_prefix()
            .expect("Não foi possível extrair o prefixo do arquivo")
            .to_str()
            .expect("Não foi possível converter o nome do arquivo para o padrão Unicode.")
            .to_string();

        let record = create_record(&key, path);
        if let Some(db) = &mut app.loaded_db {
            match db.write_all(&serialize_record(&record)) {
                Ok(_) => {
                    app.index
                        .insert(
                            key,
                            db.stream_position()
                                .expect("Erro ao obter a posição atual no stream")
                                - record.header.size
                                - 8,
                        )
                        .expect("Não foi possível atualizar o índice");
                    db.write_at(&app.index.size.to_be_bytes(), 0)
                        .expect("Não foi possível atualizar o número de chaves do índice");

                    app.current_screen =
                        CurrentScreen::DatabaseLoaded(DatabasePrompt::SuccessMessage);
                }
                Err(_) => {
                    app.current_screen =
                        CurrentScreen::DatabaseLoaded(DatabasePrompt::FailureMessage);
                }
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
                    app.index = BTree::new();
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
