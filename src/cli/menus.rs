use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, DatabaseCommands},
    btree::BTree,
    pager::Pager,
};

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
        KeyCode::Enter => {
            if app.option_highlighted == n {
                app.option_highlighted = 0;
                return Some(DatabaseListingOptions::Exit);
            } else {
                let chosen_db = app.databases[app.option_highlighted as usize].as_str();
                app.loaded_db = Some(Pager::new(chosen_db));

                let mut pager = Pager::new(chosen_db);
                match pager.read_at(0, 8) {
                    Ok(offset) => {
                        let offset_bytes = offset.try_into()
                                        .expect("Vetor de bytes contendo offset da raiz deve ter exatamente 8 bytes.");
                        let root_offset = u64::from_be_bytes(offset_bytes);
                        app.index = BTree { root: if root_offset == 0 { None } else { Some(root_offset) } };
                        app.loaded_db = Some(pager);
                    }
                    Err(_) => {}
                }

                return Some(DatabaseListingOptions::ChooseDb);
            }
        }
        KeyCode::Esc => return Some(DatabaseListingOptions::Exit),
        _ => {}
    }

    return None;
}

pub fn database_commands(key: KeyEvent, app: &mut App) -> Option<DatabaseCommands> {
    let op = app.option_highlighted;
    match key.code {
        KeyCode::Down => {
            app.option_highlighted = if op == 3 { 0 } else { op + 1 };
        }
        KeyCode::Up => {
            app.option_highlighted = if op == 0 { 3 } else { op - 1 };
        }
        KeyCode::Enter => match op {
            0 => return Some(DatabaseCommands::SEARCH),
            1 => return Some(DatabaseCommands::INSERT),
            2 => return Some(DatabaseCommands::DELETE),
            3 => return Some(DatabaseCommands::CLOSE),
            _ => {}
        },
        _ => {}
    }

    return None;
}
