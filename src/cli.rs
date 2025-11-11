mod menus;
mod ui;

use ratatui::Terminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::Backend;
use std::io::*;

use crate::app::{App, CurrentScreen, MainMenu};
use crate::cli::menus::{DatabaseListingOptions, database_list};

use menus::{MainMenuOptions, main_menu};
use ui::ui;

fn event_loop(key: KeyEvent, app: &mut App) -> Option<Result<bool>> {
    match app.current_screen {
        CurrentScreen::Main(MainMenu::OptionsList) => match main_menu(key, app) {
            Some(MainMenuOptions::CreateDb) => {
                app.current_screen = CurrentScreen::Main(MainMenu::CreateDb)
            }
            Some(MainMenuOptions::LoadDb) => {
                app.option_highlighted = 0;
                app.current_screen = CurrentScreen::DatabaseList;
            }
            Some(MainMenuOptions::Exit) => return Some(Ok(true)),
            None => {}
        },
        CurrentScreen::Main(MainMenu::CreateDb) if key.kind == KeyEventKind::Press => {
            match key.code {
                KeyCode::Char(value) => {
                    app.input.push(value);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Esc => {
                    app.input.clear();
                    app.current_screen = CurrentScreen::Main(MainMenu::OptionsList);
                }
                KeyCode::Enter if !app.input.is_empty() => match app.create_database() {
                    Ok(()) => {
                        app.input.clear();
                        app.current_screen = CurrentScreen::Main(MainMenu::SuccessMessage);
                        app.list_databases();
                    }
                    Err(e) => panic!("Error: {e:?}"),
                },
                _ => {}
            }
        }
        CurrentScreen::Main(MainMenu::SuccessMessage) => match key.code {
            KeyCode::Enter | KeyCode::Esc => {
                app.current_screen = CurrentScreen::Main(MainMenu::OptionsList);
            }
            _ => {}
        },
        CurrentScreen::DatabaseList => {
            match database_list(key, app) {
                Some(DatabaseListingOptions::ChooseDb) => {
                    // Parse and verify database file header
                    // Build B-Tree for indexing
                    // Change current screen
                }
                Some(DatabaseListingOptions::Exit) => {
                    app.option_highlighted = 0;
                    app.current_screen = CurrentScreen::Main(MainMenu::OptionsList);
                }
                None => {}
            }
        }
        _ => {}
    }

    None
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match event_loop(key, app) {
                Some(res) => return res,
                None => continue,
            }
        }
    }
}
