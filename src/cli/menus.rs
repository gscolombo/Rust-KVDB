use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{self, App};

pub enum MainMenuOptions {
    CreateDb,
    LoadDb,
    Exit,
}

pub fn main_menu(key: KeyEvent, app: &mut App) -> Option<MainMenuOptions> {
    match key.code {
        KeyCode::Up => {
            if app.option_highlighted == 0 {
                app.option_highlighted = 2;
            } else {
                app.option_highlighted -= 1;
            }
        }
        KeyCode::Down => {
            if app.option_highlighted == 2 {
                app.option_highlighted = 0;
            } else {
                app.option_highlighted += 1;
            }
        }
        KeyCode::Enter => {
            match app.option_highlighted {
                0 => return Some(MainMenuOptions::CreateDb),
                1 => return Some(MainMenuOptions::LoadDb),
                2 => return Some(MainMenuOptions::Exit),
                _ => return None,
            };
        }
        _ => {}
    }

    return None;
}

pub enum DatabaseListingOptions {
    ChooseDb,
    Exit,
}

pub fn database_list(key: KeyEvent, app: &mut App) -> Option<DatabaseListingOptions> {
    let n = app.databases.len() as u8;
    match key.code {
        KeyCode::Up => {
            if app.option_highlighted == 0 {
                app.option_highlighted = n;
            } else {
                app.option_highlighted -= 1;
            }
        }
        KeyCode::Down => {
            if app.option_highlighted == n {
                app.option_highlighted = 0;
            } else {
                app.option_highlighted += 1;
            }
        }
        KeyCode::Enter => match app.option_highlighted {
            n => return Some(DatabaseListingOptions::Exit),
            _ => return Some(DatabaseListingOptions::ChooseDb),
        },
        KeyCode::Esc => return Some(DatabaseListingOptions::Exit),
        _ => {}
    }

    return None;
}
